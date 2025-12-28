//! ===========================================================================
//! Unit09 – Update Repo Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/update_repo.rs
//!
//! This instruction updates metadata and flags on an existing `Repo` account.
//!
//! A repository may evolve over time as the underlying codebase moves,
//! rebrands, or changes maintenance strategy. This instruction allows the
//! repository authority to:
//!
//! - change the human-readable name
//! - update the canonical URL
//! - adjust tags used for discovery
//! - toggle `is_active`
//! - toggle `allow_observation`
//!
//! On success this instruction:
//! - mutates the `Repo` account fields via `Repo::apply_update`
//! - updates the `updated_at` timestamp
//! - emits:
//!     * `RepoUpdated` (always)
//!     * `RepoActivationChanged` (when `is_active` changes)
//!
//! Design notes:
//! - Only the current `Repo::authority` may perform updates
//! - Deployment must be active (`Config`) and writable (`Lifecycle`)
//! - All arguments are optional; only provided fields are updated
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::{RepoActivationChanged, RepoUpdated};
use crate::state::{Config, Lifecycle, Repo};

/// Arguments for the `update_repo` instruction.
///
/// All fields are optional. If a field is `None`, the corresponding value on
/// the `Repo` account is left unchanged.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UpdateRepoArgs {
    /// Optional new human-readable repository name.
    ///
    /// Example: "unit09-solana-core"
    pub name: Option<String>,

    /// Optional new canonical URL to the codebase.
    ///
    /// Example: "https://github.com/unit09-labs/unit09"
    pub url: Option<String>,

    /// Optional new tags for search and discovery.
    ///
    /// Example: "solana,anchor,protocol"
    pub tags: Option<String>,

    /// Optional new activation flag.
    ///
    /// - true  => repository is active and can be observed
    /// - false => repository should be treated as inactive
    pub is_active: Option<bool>,

    /// Optional new observation permission.
    ///
    /// - true  => automated observation is allowed
    /// - false => automated observation should be disabled
    pub allow_observation: Option<bool>,
}

/// Accounts required for the `update_repo` instruction.
#[derive(Accounts)]
pub struct UpdateRepo<'info> {
    /// Authority that owns this repository entry.
    ///
    /// Must match `repo.authority`. Only this signer can update the repo.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Global configuration account.
    ///
    /// Used to ensure the deployment is active. Admin authority is not
    /// required to update a repository; repository-level authority is used.
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

    /// Repository to be updated.
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

    /// System program (required by Anchor for some flows).
    pub system_program: Program<'info, System>,

    /// Clock sysvar used for timestamps.
    pub clock: Sysvar<'info, Clock>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Entry point for the `update_repo` instruction.
///
/// Steps:
/// 1. Ensure lifecycle allows writes and config is active.
/// 2. Ensure caller is the repository authority.
/// 3. Perform early string length validation on provided values.
/// 4. Call `Repo::apply_update` to mutate fields.
/// 5. Emit `RepoUpdated` and optionally `RepoActivationChanged`.
pub fn handle(ctx: Context<UpdateRepo>, args: UpdateRepoArgs) -> Result<()> {
    let UpdateRepo {
        authority: _,
        mut config,
        mut lifecycle,
        mut repo,
        system_program: _,
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

    // `has_one = authority` in the account constraint already enforces that
    // the signer is the repo authority, but we keep an explicit check for
    // clarity and defensiveness in case constraints are modified later.
    repo.assert_authority(&ctx.accounts.authority)?;

    // -----------------------------------------------------------------------
    // Early validation on provided arguments
    // -----------------------------------------------------------------------

    if let Some(ref name) = args.name {
        if name.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if name.len() > Repo::MAX_NAME_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if let Some(ref url) = args.url {
        if url.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if url.len() > Repo::MAX_URL_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    if let Some(ref tags) = args.tags {
        if tags.len() > Repo::MAX_TAGS_LEN {
            return err!(Unit09Error::StringTooLong);
        }
    }

    // -----------------------------------------------------------------------
    // Detect activation changes for event emission
    // -----------------------------------------------------------------------

    let previous_is_active = repo.is_active;

    // -----------------------------------------------------------------------
    // Apply updates to Repo
    // -----------------------------------------------------------------------

    repo.apply_update(
        args.name,
        args.url,
        args.tags,
        args.is_active,
        args.allow_observation,
        clock_ref,
    )?;

    // -----------------------------------------------------------------------
    // Emit RepoUpdated event (always)
    // -----------------------------------------------------------------------

    emit!(RepoUpdated {
        repo: repo.key(),
        url: repo.url.clone(),
    });

    // -----------------------------------------------------------------------
    // Emit RepoActivationChanged event (only when is_active changed)
    // -----------------------------------------------------------------------

    if repo.is_active != previous_is_active {
        emit!(RepoActivationChanged {
            repo: repo.key(),
            is_active: repo.is_active,
            updated_at: repo.updated_at,
        });
    }

    Ok(())
}
