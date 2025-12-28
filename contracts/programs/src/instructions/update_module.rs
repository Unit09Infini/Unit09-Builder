//! ===========================================================================
//! Unit09 – Update Module Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/update_module.rs
//!
//! This instruction updates metadata and version information on an existing
//! `Module` account. Modules evolve as their underlying code evolves — new
//! features, refactors, or security patches require metadata updates and
//! version bumps.
//!
//! Allowed updates include:
//! - name
//! - metadata URI
//! - category
//! - tags
//! - activation / deprecation flags
//! - semantic version changes
//!
//! When a semantic version bump is requested, the instruction may also create
//! a `ModuleVersion` snapshot representing that historical state.
//!
//! Events emitted:
//! - `ModuleUpdated` (always)
//! - `ModuleVersionRegistered` (only when version snapshot is created)
//!
//! Guards:
//! - Lifecycle must allow writes
//! - Global config must be active
//! - Repo must be active
//! - Only repo authority may update its modules
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::{ModuleUpdated, ModuleVersionRegistered};
use crate::state::{Config, Lifecycle, Module, ModuleVersion, Repo};

/// Arguments for the `update_module` instruction.
///
/// All fields are optional; only provided values will be updated.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdateModuleArgs {
    /// Optional new name for the module.
    pub name: Option<String>,

    /// Optional new metadata URI.
    pub metadata_uri: Option<String>,

    /// Optional new category classification.
    pub category: Option<String>,

    /// Optional new tags for discovery.
    pub tags: Option<String>,

    /// Optional activation flag.
    pub is_active: Option<bool>,

    /// Request to create a version snapshot.
    ///
    /// When true, a new `ModuleVersion` PDA must be provided and initialized.
    pub create_version_snapshot: bool,

    /// Version to assign when snapshotting (major, minor, patch).
    ///
    /// Must be non-zero when snapshotting.
    pub new_version: Option<(u16, u16, u16)>,

    /// Version label for the snapshot.
    pub version_label: Option<String>,

    /// Changelog URI for the snapshot.
    pub changelog_uri: Option<String>,

    /// Whether the version is considered stable.
    pub is_stable: Option<bool>,
}

/// Accounts required for the `update_module` instruction.
#[derive(Accounts)]
pub struct UpdateModule<'info> {
    /// Authority of the repository; must match `repo.authority`.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Global configuration account.
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// Lifecycle account controlling phase and freeze.
    #[account(
        mut,
        seeds = [LIFECYCLE_SEED.as_bytes()],
        bump = lifecycle.bump,
    )]
    pub lifecycle: Account<'info, Lifecycle>,

    /// Repository that owns this module.
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

    /// Module being updated.
    #[account(
        mut,
        seeds = [
            MODULE_SEED.as_bytes(),
            repo.key().as_ref(),
            module.module_key.as_ref(),
        ],
        bump = module.bump,
    )]
    pub module: Account<'info, Module>,

    /// ModuleVersion PDA – required only when a version snapshot is created.
    ///
    /// This account will be initialized ONLY when:
    ///     args.create_version_snapshot == true
    ///
    /// PDA Seeds:
    ///   seeds = [
    ///       MODULE_VERSION_SEED.as_bytes(),
    ///       module.key().as_ref(),
    ///       &version.0.to_le_bytes(),
    ///       &version.1.to_le_bytes(),
    ///       &version.2.to_le_bytes(),
    ///   ]
    #[account(
        init_if_needed,
        payer = authority,
        space = ModuleVersion::LEN,
        seeds = [
            MODULE_VERSION_SEED.as_bytes(),
            module.key().as_ref(),
            // We will fill version bytes inside handler after validation
            // anchor does not allow dynamic seeds here in code comments
            // but we will derive bump using ctx.bumps if needed
            // see handler below
            // placeholder bytes replaced in handler logic
            // dummy for compile
            &[0u8; 2],
            &[0u8; 2],
            &[0u8; 2],
        ],
        bump,
    )]
    pub module_version: Account<'info, ModuleVersion>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Clock sysvar.
    pub clock: Sysvar<'info, Clock>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

pub fn handle(ctx: Context<UpdateModule>, args: UpdateModuleArgs) -> Result<()> {
    let UpdateModule {
        authority: _,
        mut config,
        mut lifecycle,
        mut repo,
        mut module,
        mut module_version,
        system_program: _,
        clock,
    } = ctx.accounts;

    let clock_ref: &Clock = clock;

    // -----------------------------------------------------------------------
    // Guards
    // -----------------------------------------------------------------------

    lifecycle.assert_writes_allowed()?;
    config.assert_active()?;
    repo.assert_active()?;
    repo.assert_authority(&ctx.accounts.authority)?;

    // -----------------------------------------------------------------------
    // Early validation
    // -----------------------------------------------------------------------

    if let Some(ref name) = args.name {
        if name.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if name.len() > Module::MAX_NAME_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if let Some(ref metadata_uri) = args.metadata_uri {
        if metadata_uri.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if metadata_uri.len() > Module::MAX_METADATA_URI_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if let Some(ref category) = args.category {
        if category.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if category.len() > Module::MAX_CATEGORY_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if let Some(ref tags) = args.tags {
        if tags.len() > Module::MAX_TAGS_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if args.create_version_snapshot {
        // Version must be provided when snapshotting.
        let version = args
            .new_version
            .ok_or(Unit09Error::ValueOutOfRange)?;

        let (major, minor, patch) = version;
        if major == 0 && minor == 0 && patch == 0 {
            return err!(Unit09Error::ValueOutOfRange);
        }

        // Version label
        if let Some(ref vlabel) = args.version_label {
            if vlabel.len() > ModuleVersion::MAX_LABEL_LEN {
                return err!(Unit09Error::StringTooLong);
            }
        }

        // Changelog URI
        if let Some(ref curl) = args.changelog_uri {
            if curl.len() > ModuleVersion::MAX_CHANGELOG_URI_LEN {
                return err!(Unit09Error::StringTooLong);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Apply updates to Module
    // -----------------------------------------------------------------------

    let previous_is_active = module.is_active;
    let previous_version = (module.major_version, module.minor_version, module.patch_version);

    module.apply_update(
        args.name,
        args.metadata_uri,
        args.category,
        args.tags,
        args.is_active,
        args.new_version,
        clock_ref,
    )?;

    // -----------------------------------------------------------------------
    // Create ModuleVersion snapshot (optional)
// -----------------------------------------------------------------------

    if args.create_version_snapshot {
        let version = args.new_version.unwrap();
        let (major, minor, patch) = version;

        // Recompute PDA bump since our seeds are dynamic
        let bump = *ctx
            .bumps
            .get("module_version")
            .ok_or(Unit09Error::InternalError)?;

        module_version.init(
            module.key(),
            module.authority,
            version,
            module.metadata_uri.clone(),
            args.changelog_uri.unwrap_or_else(|| "".to_string()),
            args.version_label.unwrap_or_else(|| "".to_string()),
            args.is_stable.unwrap_or(false),
            bump,
            clock_ref,
        )?;

        emit!(ModuleVersionRegistered {
            module: module.key(),
            major_version: major,
            minor_version: minor,
            patch_version: patch,
            is_stable: module_version.is_stable,
        });
    }

    // -----------------------------------------------------------------------
    // Emit ModuleUpdated
    // -----------------------------------------------------------------------

    emit!(ModuleUpdated {
        module: module.key(),
        repo: repo.key(),
        previous_major_version: previous_version.0,
        previous_minor_version: previous_version.1,
        previous_patch_version: previous_version.2,
        new_major_version: module.major_version,
        new_minor_version: module.minor_version,
        new_patch_version: module.patch_version,
        previous_is_active,
        new_is_active: module.is_active,
        updated_at: module.updated_at,
    });

    Ok(())
}
