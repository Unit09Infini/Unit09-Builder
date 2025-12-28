//! ===========================================================================
//! Unit09 – Register Module Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/register_module.rs
//!
//! This instruction registers a new `Module` under an existing `Repo`.
//!
//! Conceptually, a `Module` is a runnable, reusable unit of logic that Unit09
//! extracts from a real-world codebase. It can represent:
//! - a Solana program entry module
//! - a reusable instruction set
//! - a shared library or abstraction
//!
//! On success this instruction:
//! - creates and initializes a `Module` PDA
//! - optionally creates a `ModuleVersion` PDA for the initial version
//! - increments per-repo module counters and global module metrics
//! - emits `ModuleRegistered` and `ModuleVersionRegistered` events
//!
//! Guards:
//! - Lifecycle must allow writes (`Lifecycle::assert_writes_allowed`)
//! - Global config must be active (`Config::assert_active`)
//! - Target repo must be active (`Repo::assert_active`)
//! - Only the repo authority can register modules for that repo
//!
//! PDA layout:
//! - Module:
//!     seeds = [MODULE_SEED, repo.key().as_ref(), module_key.as_ref()]
//! - ModuleVersion (optional initial snapshot):
//!     seeds = [MODULE_VERSION_SEED, module.key().as_ref(),
//!              major_version.to_le_bytes(), minor_version.to_le_bytes(),
//!              patch_version.to_le_bytes()]
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::{ModuleRegistered, ModuleVersionRegistered};
use crate::state::{Config, Lifecycle, Metrics, Module, ModuleVersion, Repo};

/// Arguments for the `register_module` instruction.
///
/// The caller chooses a `module_key` that, together with the repo, identifies
/// this module uniquely for PDA derivation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RegisterModuleArgs {
    /// Arbitrary key used together with `MODULE_SEED` and the repo key
    /// to derive the `Module` PDA.
    pub module_key: Pubkey,

    /// Human-readable module name.
    ///
    /// Example: "unit09-router", "metrics-indexer"
    pub name: String,

    /// Off-chain metadata URI for this module.
    ///
    /// Example: "https://unit09.org/metadata/modules/router.json"
    pub metadata_uri: String,

    /// Category classification for this module.
    ///
    /// Example:
    /// - "program"
    /// - "library"
    /// - "indexer"
    /// - "worker"
    pub category: String,

    /// Tags used for search and discovery.
    ///
    /// Example: "solana,anchor,token,module"
    pub tags: String,

    /// Initial semantic version for this module.
    ///
    /// (major, minor, patch)
    pub version: (u16, u16, u16),

    /// Optional version label, used when creating an initial
    /// `ModuleVersion` snapshot.
    ///
    /// Example: "alpha", "beta", "v1-initial"
    pub version_label: String,

    /// Optional version-specific changelog URI.
    ///
    /// Example: "https://unit09.org/changelog/module-x/v1.0.0"
    pub changelog_uri: String,

    /// Whether this initial version is considered stable.
    pub is_stable: bool,

    /// Whether to create a `ModuleVersion` snapshot for the initial version.
    ///
    /// If false, only the `Module` account is created.
    pub create_initial_version_snapshot: bool,
}

/// Accounts required for the `register_module` instruction.
#[derive(Accounts)]
pub struct RegisterModule<'info> {
    /// Payer for the newly created accounts.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Authority of the repository; must match `repo.authority`.
    ///
    /// Only the repo authority can register new modules under that repo.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Global configuration account.
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// Lifecycle account controlling high-level phases and freezes.
    #[account(
        mut,
        seeds = [LIFECYCLE_SEED.as_bytes()],
        bump = lifecycle.bump,
    )]
    pub lifecycle: Account<'info, Lifecycle>,

    /// Global metrics account.
    #[account(
        mut,
        seeds = [METRICS_SEED.as_bytes()],
        bump = metrics.bump,
    )]
    pub metrics: Account<'info, Metrics>,

    /// Repository under which this module is being registered.
    ///
    /// PDA:
    ///   seeds = [REPO_SEED.as_bytes(), repo.repo_key.as_ref()]
    ///   bump  = repo.bump
    #[account(
        mut,
        seeds = [
            REPO_SEED.as_bytes(),
            repo.repo_key.as_ref(),
        ],
        bump = repo.bump,
        has_one = authority @ Unit09Error::InvalidAuthority,
    )]
    pub repo: Account<'info, Repo>,

    /// Module account to be created.
    ///
    /// PDA:
    ///   seeds = [
    ///       MODULE_SEED.as_bytes(),
    ///       repo.key().as_ref(),
    ///       args.module_key.as_ref(),
    ///   ]
    ///   bump  = module.bump
    #[account(
        init,
        payer = payer,
        space = Module::LEN,
        seeds = [
            MODULE_SEED.as_bytes(),
            repo.key().as_ref(),
            args.module_key.as_ref(),
        ],
        bump,
    )]
    pub module: Account<'info, Module>,

    /// Optional module version snapshot for the initial version.
    ///
    /// When `args.create_initial_version_snapshot` is true, this account
    /// must be provided and will be initialized. When false, it is unused.
    ///
    /// PDA:
    ///   seeds = [
    ///       MODULE_VERSION_SEED.as_bytes(),
    ///       module.key().as_ref(),
    ///       &args.version.0.to_le_bytes(),
    ///       &args.version.1.to_le_bytes(),
    ///       &args.version.2.to_le_bytes(),
    ///   ]
    ///   bump  = module_version.bump
    #[account(
        init_if_needed,
        payer = payer,
        space = ModuleVersion::LEN,
        seeds = [
            MODULE_VERSION_SEED.as_bytes(),
            module.key().as_ref(),
            &args.version.0.to_le_bytes(),
            &args.version.1.to_le_bytes(),
            &args.version.2.to_le_bytes(),
        ],
        bump,
    )]
    pub module_version: Account<'info, ModuleVersion>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,

    /// Clock sysvar for timestamps.
    pub clock: Sysvar<'info, Clock>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Entry point for the `register_module` instruction.
///
/// Steps:
/// 1. Check lifecycle and config state.
/// 2. Ensure repo is active and authority matches.
/// 3. Validate incoming strings and version.
/// 4. Initialize `Module` account.
/// 5. Optionally initialize `ModuleVersion` snapshot.
/// 6. Update repo and metrics counters.
/// 7. Emit events.
pub fn handle(ctx: Context<RegisterModule>, args: RegisterModuleArgs) -> Result<()> {
    let RegisterModule {
        payer: _,
        authority,
        mut config,
        mut lifecycle,
        mut metrics,
        mut repo,
        mut module,
        mut module_version,
        system_program: _,
        rent: _,
        clock,
    } = ctx.accounts;

    let clock_ref: &Clock = clock;

    // -----------------------------------------------------------------------
    // Lifecycle and configuration guards
    // -----------------------------------------------------------------------

    lifecycle.assert_writes_allowed()?;
    config.assert_active()?;
    repo.assert_active()?;

    // `has_one = authority` already enforces authority, but we check again
    // defensively for clarity.
    repo.assert_authority(authority)?;

    // -----------------------------------------------------------------------
    // Early validation on provided arguments
    // -----------------------------------------------------------------------

    // Name
    if args.name.is_empty() {
        return err!(Unit09Error::StringEmpty);
    }
    if args.name.len() > Module::MAX_NAME_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Metadata URI
    if args.metadata_uri.is_empty() {
        return err!(Unit09Error::StringEmpty);
    }
    if args.metadata_uri.len() > Module::MAX_METADATA_URI_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Category
    if args.category.is_empty() {
        return err!(Unit09Error::StringEmpty);
    }
    if args.category.len() > Module::MAX_CATEGORY_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Tags
    if args.tags.len() > Module::MAX_TAGS_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Version label (for ModuleVersion)
    if args.version_label.len() > ModuleVersion::MAX_LABEL_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Changelog URI (for ModuleVersion)
    if args.changelog_uri.len() > ModuleVersion::MAX_CHANGELOG_URI_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // Version sanity
    let version = args.version;
    {
        let (major, minor, patch) = version;
        if major == 0 && minor == 0 && patch == 0 {
            return err!(Unit09Error::ValueOutOfRange);
        }
    }

    // -----------------------------------------------------------------------
    // Derive PDA bumps from Anchor context
    // -----------------------------------------------------------------------

    let module_bump = *ctx.bumps.get("module").ok_or(Unit09Error::InternalError)?;

    // For `init_if_needed` we only use the bump if we actually create/init.
    let module_version_bump = ctx.bumps.get("module_version").copied();

    // -----------------------------------------------------------------------
    // Initialize Module account
    // -----------------------------------------------------------------------

    module.init(
        args.module_key,
        repo.key(),
        authority.key(),
        args.name,
        args.metadata_uri,
        args.category,
        args.tags,
        version,
        module_bump,
        clock_ref,
    )?;

    // -----------------------------------------------------------------------
    // Optionally initialize ModuleVersion snapshot
    // -----------------------------------------------------------------------

    if args.create_initial_version_snapshot {
        let bump = module_version_bump.ok_or(Unit09Error::InternalError)?;

        module_version.init(
            module.key(),
            authority.key(),
            version,
            module.metadata_uri.clone(),
            args.changelog_uri,
            args.version_label,
            args.is_stable,
            bump,
            clock_ref,
        )?;

        emit!(ModuleVersionRegistered {
            module: module.key(),
            major_version: version.0,
            minor_version: version.1,
            patch_version: version.2,
            is_stable: module_version.is_stable,
        });
    }

    // -----------------------------------------------------------------------
    // Update per-repo counters and global metrics
    // -----------------------------------------------------------------------

    repo.increment_module_count()?;
    repo.updated_at = clock_ref.unix_timestamp;

    metrics.increment_modules()?;
    metrics.updated_at = clock_ref.unix_timestamp;

    // -----------------------------------------------------------------------
    // Emit ModuleRegistered event
    // -----------------------------------------------------------------------

    emit!(ModuleRegistered {
        module: module.key(),
        repo: repo.key(),
        owner: module.authority,
        category: module.category.clone(),
        major_version: module.major_version,
        minor_version: module.minor_version,
        patch_version: module.patch_version,
    });

    Ok(())
}
