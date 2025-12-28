//! ===========================================================================
//! Unit09 – Link Module To Repo Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/link_module_to_repo.rs
//!
//! Purpose
//! -------
//! This instruction creates (or refreshes) an explicit linkage between a
//! `Module` and a `Repo` in the Unit09 system.
//
//! Conceptually, a `Module` represents a reusable, runnable unit of logic.
//! A `Repo` represents a real-world codebase. A single module may:
//! - originate from one repo
//! - be reused by multiple other repos
//! - be indexed differently by different consumers
//!
//! Instead of forcing a one-to-one mapping between module and repo,
//! this instruction writes a dedicated link account that encodes:
//! - which module is linked
//! - which repo it is linked to
//! - who linked it
//! - whether this link is considered "primary" or "secondary"
//! - optional notes useful for off-chain indexers or UIs
//!
//! On success this instruction:
//! - ensures lifecycle and config allow writes
//! - ensures the target repo is active
//! - ensures the signer is allowed to link this module
//! - initializes or updates a `ModuleRepoLink` PDA
//! - emits a `ModuleLinkedToRepo` event (for indexers and dashboards)
//!
//! PDA layout
//! ----------
//! - `ModuleRepoLink`:
//!     seeds = [
//!         MODULE_REPO_LINK_SEED,
//!         module.key().as_ref(),
//!         repo.key().as_ref(),
//!     ]
//!
//! Design notes
//! ------------
//! - A link is idempotent: calling this multiple times for the same
//!   (module, repo) pair will simply refresh metadata and timestamps.
//! - This instruction does *not* move a module from one repo to another;
//!   that would require a different migration flow. It only records an
//!   association for discovery and analytics.
//! - Authorization is granted to:
//!     * the module authority, OR
//!     * the repo authority
//!   so that either side can manage their own linkage graph.
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::ModuleLinkedToRepo;
use crate::state::{Config, Lifecycle, Module, ModuleRepoLink, Repo};

/// Arguments for the `link_module_to_repo` instruction.
///
/// The `(module, repo)` pair is implied by the accounts; the arguments
/// only carry link-specific metadata.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct LinkModuleToRepoArgs {
    /// Whether this link should be treated as the primary association
    /// between this module and the given repo.
    ///
    /// Off-chain tools can use this flag to distinguish:
    /// - a "home" repo for the module
    /// - secondary or downstream repos that reuse it
    pub is_primary: bool,

    /// Optional free-form notes for off-chain indexers or dashboards.
    ///
    /// Example:
    /// - "used for indexing events only"
    /// - "forked variant with modified filters"
    pub notes: String,
}

/// Accounts required for the `link_module_to_repo` instruction.
#[derive(Accounts)]
pub struct LinkModuleToRepo<'info> {
    /// Payer for the link account initialization (if needed).
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Signer authorized to create or update the link.
    ///
    /// This must be either:
    /// - the module authority, OR
    /// - the repo authority
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Global configuration account.
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// Lifecycle account controlling global write permissions.
    #[account(
        mut,
        seeds = [LIFECYCLE_SEED.as_bytes()],
        bump = lifecycle.bump,
    )]
    pub lifecycle: Account<'info, Lifecycle>,

    /// Repository to which the module is being linked.
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
    )]
    pub repo: Account<'info, Repo>,

    /// Module that is being linked to the target repo.
    ///
    /// PDA:
    ///   seeds = [
    ///       MODULE_SEED.as_bytes(),
    ///       module.module_key.as_ref(),
    ///   ]
    ///   bump  = module.bump
    ///
    /// Note:
    /// - The module PDA is derived only from `MODULE_SEED` and `module_key`
    ///   so that it can be associated with multiple repos via link accounts.
    #[account(
        mut,
        seeds = [
            MODULE_SEED.as_bytes(),
            module.module_key.as_ref(),
        ],
        bump = module.bump,
    )]
    pub module: Account<'info, Module>,

    /// Link account between the module and the repo.
    ///
    /// PDA:
    ///   seeds = [
    ///       MODULE_REPO_LINK_SEED.as_bytes(),
    ///       module.key().as_ref(),
    ///       repo.key().as_ref(),
    ///   ]
    ///   bump  = link.bump
    ///
    /// This account may already exist; in that case it will be updated
    /// rather than reinitialized.
    #[account(
        init_if_needed,
        payer = payer,
        space = ModuleRepoLink::LEN,
        seeds = [
            MODULE_REPO_LINK_SEED.as_bytes(),
            module.key().as_ref(),
            repo.key().as_ref(),
        ],
        bump,
    )]
    pub link: Account<'info, ModuleRepoLink>,

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

/// Entry point for the `link_module_to_repo` instruction.
///
/// Steps:
/// 1. Enforce lifecycle and config guards.
/// 2. Enforce repo activity.
/// 3. Enforce that the signer is either module or repo authority.
/// 4. Validate notes length.
/// 5. Initialize or update `ModuleRepoLink`.
/// 6. Emit `ModuleLinkedToRepo` event.
pub fn handle(ctx: Context<LinkModuleToRepo>, args: LinkModuleToRepoArgs) -> Result<()> {
    let LinkModuleToRepo {
        payer: _,
        authority,
        mut config,
        mut lifecycle,
        mut repo,
        mut module,
        mut link,
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

    // -----------------------------------------------------------------------
    // Authorization: signer must be module or repo authority
    // -----------------------------------------------------------------------

    let signer_key = authority.key();

    let is_module_authority = signer_key == module.authority;
    let is_repo_authority = signer_key == repo.authority;

    if !is_module_authority && !is_repo_authority {
        return err!(Unit09Error::InvalidAuthority);
    }

    // -----------------------------------------------------------------------
    // Basic validation for notes
    // -----------------------------------------------------------------------

    if args.notes.len() > ModuleRepoLink::MAX_NOTES_LEN {
        return err!(Unit09Error::StringTooLong);
    }

    // -----------------------------------------------------------------------
    // Derive bump from Anchor context
    // -----------------------------------------------------------------------

    let link_bump = *ctx.bumps.get("link").ok_or(Unit09Error::InternalError)?;

    // -----------------------------------------------------------------------
    // Initialize or update link account
    // -----------------------------------------------------------------------

    let now = clock_ref.unix_timestamp;

    // If this is a fresh account (default/zeroed), we treat it as init.
    let is_new = link.module == Pubkey::default() && link.repo == Pubkey::default();

    if is_new {
        // First-time initialization of the link.
        link.module = module.key();
        link.repo = repo.key();
        link.linked_by = signer_key;
        link.is_primary = args.is_primary;
        link.notes = args.notes;
        link.created_at = now;
        link.updated_at = now;
        link.schema_version = CURRENT_SCHEMA_VERSION;
        link.bump = link_bump;
        link.reserved = [0u8; 63];
    } else {
        // Existing link: refresh flags and notes.
        link.is_primary = args.is_primary;
        link.notes = args.notes;
        link.linked_by = signer_key;
        link.updated_at = now;
    }

    // -----------------------------------------------------------------------
    // Emit ModuleLinkedToRepo event
    // -----------------------------------------------------------------------

    emit!(ModuleLinkedToRepo {
        module: module.key(),
        repo: repo.key(),
        linked_by: signer_key,
        is_primary: link.is_primary,
        updated_at: link.updated_at,
    });

    Ok(())
}
