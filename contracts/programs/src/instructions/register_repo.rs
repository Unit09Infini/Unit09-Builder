//! ===========================================================================
//! Unit09 – Register Repo Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/register_repo.rs
//!
//! This instruction registers a new repository with Unit09.
//!
//! A repository is the primary anchor for:
//! - tracking a real-world codebase
//! - attaching generated modules
//! - aggregating observation stats
//!
//! On success this instruction:
//! - creates and initializes a `Repo` PDA
//! - increments the global `Metrics::total_repos` counter
//! - emits a `RepoRegistered` event
//!
//! Design notes:
//! - Any signer can become a repository authority (no admin gate by default)
//! - The deployment must be active (`Config`) and writable (`Lifecycle`)
//! - Basic string and bounds validation is handled by `Repo::init`
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::RepoRegistered;
use crate::state::{Config, Lifecycle, Metrics, Repo};

/// Arguments for the `register_repo` instruction.
///
/// The caller chooses a `repo_key` that will be used in PDA derivation.
/// Common patterns:
/// - hash of a repository URL
/// - random key generated locally
/// - wallet public key for a personal code space
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RegisterRepoArgs {
    /// Arbitrary key used together with `REPO_SEED` to derive the `Repo` PDA.
    pub repo_key: Pubkey,

    /// Human-readable repository name.
    ///
    /// Example: "unit09-solana-core"
    pub name: String,

    /// URL to the codebase.
    ///
    /// Example: "https://github.com/unit09-labs/unit09"
    pub url: String,

    /// Optional tags for search and discovery.
    ///
    /// Example: "solana,anchor,protocol"
    pub tags: String,

    /// Whether automated observation is allowed for this repository.
    pub allow_observation: bool,
}

/// Accounts required for the `register_repo` instruction.
#[derive(Accounts)]
pub struct RegisterRepo<'info> {
    /// Payer for the newly created `Repo` account.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Authority that will own this repository entry.
    ///
    /// This key is stored as `Repo::authority` and will be required for
    /// future updates to this repository.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Global configuration account.
    ///
    /// Used to ensure the deployment is active and to access global limits
    /// if needed. The admin stored here is NOT required for repo creation.
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// Lifecycle account controlling high-level operation and freezes.
    #[account(
        mut,
        seeds = [LIFECYCLE_SEED.as_bytes()],
        bump = lifecycle.bump,
    )]
    pub lifecycle: Account<'info, Lifecycle>,

    /// Global metrics account that aggregates deployment-wide counters.
    #[account(
        mut,
        seeds = [METRICS_SEED.as_bytes()],
        bump = metrics.bump,
    )]
    pub metrics: Account<'info, Metrics>,

    /// The repository account to be created.
    ///
    /// PDA:
    ///   seeds = [REPO_SEED.as_bytes(), args.repo_key.as_ref()]
    ///   bump  = repo.bump
    #[account(
        init,
        payer = payer,
        space = Repo::LEN,
        seeds = [
            REPO_SEED.as_bytes(),
            args.repo_key.as_ref(),
        ],
        bump,
    )]
    pub repo: Account<'info, Repo>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,

    /// Clock sysvar used for timestamps.
    pub clock: Sysvar<'info, Clock>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Entry point for the `register_repo` instruction.
///
/// Steps:
/// 1. Ensure lifecycle allows write operations.
/// 2. Ensure global config is active (if enforced).
/// 3. Initialize the `Repo` account with validated metadata.
/// 4. Increment global repository counter in `Metrics`.
/// 5. Emit `RepoRegistered` event.
pub fn handle(ctx: Context<RegisterRepo>, args: RegisterRepoArgs) -> Result<()> {
    let RegisterRepo {
        payer: _,
        authority,
        mut config,
        mut lifecycle,
        mut metrics,
        mut repo,
        system_program: _,
        rent: _,
        clock,
    } = ctx.accounts;

    let clock_ref: &Clock = clock;

    // -----------------------------------------------------------------------
    // Lifecycle and configuration guards
    // -----------------------------------------------------------------------

    // Ensure writes are allowed for this deployment.
    lifecycle.assert_writes_allowed()?;

    // Ensure the configuration is currently active.
    config.assert_active()?;

    // -----------------------------------------------------------------------
    // Basic early argument validation (string length sanity checks)
    // -----------------------------------------------------------------------
    //
    // Detailed validation is also performed inside `Repo::init`, but we
    // perform simple checks here to fail fast and avoid unnecessary work.

    if args.name.is_empty() {
        return err!(Unit09Error::StringEmpty);
    }
    if args.name.len() > Repo::MAX_NAME_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    if args.url.is_empty() {
        return err!(Unit09Error::StringEmpty);
    }
    if args.url.len() > Repo::MAX_URL_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    if args.tags.len() > Repo::MAX_TAGS_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // -----------------------------------------------------------------------
    // Derive bump from Anchor context
    // -----------------------------------------------------------------------

    let repo_bump = *ctx.bumps.get("repo").ok_or(Unit09Error::InternalError)?;

    // -----------------------------------------------------------------------
    // Initialize Repo account
    // -----------------------------------------------------------------------

    repo.init(
        args.repo_key,
        authority.key(),
        args.name,
        args.url,
        args.tags,
        args.allow_observation,
        repo_bump,
        clock_ref,
    )?;

    // -----------------------------------------------------------------------
    // Update global metrics
    // -----------------------------------------------------------------------

    metrics.increment_repos()?;
    metrics.updated_at = clock_ref.unix_timestamp;

    // -----------------------------------------------------------------------
    // Emit RepoRegistered event
    // -----------------------------------------------------------------------

    emit!(RepoRegistered {
        repo: repo.key(),
        owner: repo.authority,
        url: repo.url.clone(),
    });

    Ok(())
}
