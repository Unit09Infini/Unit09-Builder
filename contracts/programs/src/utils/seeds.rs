//! ===========================================================================
//! Unit09 – PDA Seed Helpers
//! Path: contracts/unit09-program/programs/unit09_program/src/utils/seeds.rs
//!
//! This module centralizes all Program Derived Address (PDA) helpers used by
//! the Unit09 program. It does not own any on-chain state; it only provides
//! deterministic seed layouts and small helper functions for:
//!
//! - deriving PDAs in a consistent way across the codebase
//! - keeping seed strings in one place (via `crate::constants`)
//! - making it easy for off-chain tools to mirror PDA derivations
//!
//! The goal is to avoid sprinkling `Pubkey::find_program_address` calls and
//! raw seed constants throughout the code. Instead, each account type should
//! have:
//!
//! - a public function that returns `(Pubkey, u8)`
//! - a clear, documented seed layout
//!
//! Off-chain tooling is expected to mirror these functions when it needs to
//! compute PDAs for fetching accounts.
//!
//! ===========================================================================

use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::constants::*;

/// Helper type alias used when returning precomputed seeds for CPI or tests.
pub type SeedSlice<'a> = &'a [&'a [u8]];

// ---------------------------------------------------------------------------
// Config / Lifecycle / Metrics / Global Metadata
// ---------------------------------------------------------------------------

/// Derive the PDA for the global `Config` account.
///
/// Seeds:
/// - `[CONFIG_SEED.as_bytes()]`
pub fn config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG_SEED.as_bytes()], program_id)
}

/// Seed slice for the `Config` PDA.
///
/// This is useful when building CPI calls or manual `invoke_signed` calls.
pub fn config_seeds<'a>(bump: u8) -> SeedSlice<'a> {
    &[CONFIG_SEED.as_bytes(), &[bump]]
}

/// Derive the PDA for the global `Lifecycle` account.
///
/// Seeds:
/// - `[LIFECYCLE_SEED.as_bytes()]`
pub fn lifecycle_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LIFECYCLE_SEED.as_bytes()], program_id)
}

pub fn lifecycle_seeds<'a>(bump: u8) -> SeedSlice<'a> {
    &[LIFECYCLE_SEED.as_bytes(), &[bump]]
}

/// Derive the PDA for the global `Metrics` account.
///
/// Seeds:
/// - `[METRICS_SEED.as_bytes()]`
pub fn metrics_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[METRICS_SEED.as_bytes()], program_id)
}

pub fn metrics_seeds<'a>(bump: u8) -> SeedSlice<'a> {
    &[METRICS_SEED.as_bytes(), &[bump]]
}

/// Derive the PDA for the `GlobalMetadata` account.
///
/// Seeds:
/// - `[GLOBAL_METADATA_SEED.as_bytes()]`
pub fn global_metadata_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[GLOBAL_METADATA_SEED.as_bytes()], program_id)
}

pub fn global_metadata_seeds<'a>(bump: u8) -> SeedSlice<'a> {
    &[GLOBAL_METADATA_SEED.as_bytes(), &[bump]]
}

// ---------------------------------------------------------------------------
// Authority
// ---------------------------------------------------------------------------

/// Derive the PDA for an `Authority` entry.
///
/// Seeds:
/// - `[AUTHORITY_SEED.as_bytes(), authority.as_ref()]`
pub fn authority_pda(program_id: &Pubkey, authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            AUTHORITY_SEED.as_bytes(),
            authority.as_ref(),
        ],
        program_id,
    )
}

pub fn authority_seeds<'a>(authority: &Pubkey, bump: u8) -> SeedSlice<'a> {
    &[
        AUTHORITY_SEED.as_bytes(),
        authority.as_ref(),
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Repo
// ---------------------------------------------------------------------------

/// Derive the PDA for a `Repo` account.
///
/// `repo_key` is an arbitrary key chosen by the caller and stored in the
/// `Repo` account. It is expected to be stable over time and unique per repo.
///
/// Seeds:
/// - `[REPO_SEED.as_bytes(), repo_key.as_ref()]`
pub fn repo_pda(program_id: &Pubkey, repo_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            REPO_SEED.as_bytes(),
            repo_key.as_ref(),
        ],
        program_id,
    )
}

pub fn repo_seeds<'a>(repo_key: &Pubkey, bump: u8) -> SeedSlice<'a> {
    &[
        REPO_SEED.as_bytes(),
        repo_key.as_ref(),
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Module
// ---------------------------------------------------------------------------

/// Derive the PDA for a `Module` account.
///
/// `module_key` is an arbitrary key chosen by the caller to represent a given
/// module. In this design, modules are scoped under a specific repo PDA so
/// the seeds include the repo address as well.
///
/// Seeds:
/// - `[MODULE_SEED.as_bytes(), repo_pubkey.as_ref(), module_key.as_ref()]`
pub fn module_pda(
    program_id: &Pubkey,
    repo_pubkey: &Pubkey,
    module_key: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            MODULE_SEED.as_bytes(),
            repo_pubkey.as_ref(),
            module_key.as_ref(),
        ],
        program_id,
    )
}

pub fn module_seeds<'a>(
    repo_pubkey: &Pubkey,
    module_key: &Pubkey,
    bump: u8,
) -> SeedSlice<'a> {
    &[
        MODULE_SEED.as_bytes(),
        repo_pubkey.as_ref(),
        module_key.as_ref(),
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Module Version
// ---------------------------------------------------------------------------

/// Derive the PDA for a `ModuleVersion` account.
///
/// The seed layout encodes module identity plus semantic version components:
///
/// Seeds:
/// - `MODULE_VERSION_SEED.as_bytes()`
/// - `module_pubkey.as_ref()`
/// - `major.to_le_bytes()`
/// - `minor.to_le_bytes()`
/// - `patch.to_le_bytes()`
pub fn module_version_pda(
    program_id: &Pubkey,
    module_pubkey: &Pubkey,
    major: u16,
    minor: u16,
    patch: u16,
) -> (Pubkey, u8) {
    let major_bytes = major.to_le_bytes();
    let minor_bytes = minor.to_le_bytes();
    let patch_bytes = patch.to_le_bytes();

    Pubkey::find_program_address(
        &[
            MODULE_VERSION_SEED.as_bytes(),
            module_pubkey.as_ref(),
            &major_bytes,
            &minor_bytes,
            &patch_bytes,
        ],
        program_id,
    )
}

pub fn module_version_seeds<'a>(
    module_pubkey: &Pubkey,
    major: u16,
    minor: u16,
    patch: u16,
    bump: u8,
) -> SeedSlice<'a> {
    let major_bytes = major.to_le_bytes();
    let minor_bytes = minor.to_le_bytes();
    let patch_bytes = patch.to_le_bytes();

    &[
        MODULE_VERSION_SEED.as_bytes(),
        module_pubkey.as_ref(),
        &major_bytes,
        &minor_bytes,
        &patch_bytes,
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Fork
// ---------------------------------------------------------------------------

/// Derive the PDA for a `Fork` account.
///
/// `fork_key` is an arbitrary key chosen by the caller to represent this
/// fork in the Unit09 personality tree.
///
/// Seeds:
/// - `[FORK_SEED.as_bytes(), fork_key.as_ref()]`
pub fn fork_pda(program_id: &Pubkey, fork_key: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            FORK_SEED.as_bytes(),
            fork_key.as_ref(),
        ],
        program_id,
    )
}

pub fn fork_seeds<'a>(fork_key: &Pubkey, bump: u8) -> SeedSlice<'a> {
    &[
        FORK_SEED.as_bytes(),
        fork_key.as_ref(),
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Module–Repo Link
// ---------------------------------------------------------------------------

/// Derive the PDA for a `ModuleRepoLink` account.
///
/// This link encodes the association between a module and a repo. A module
/// may be linked to multiple repos, and each link is tracked separately.
///
/// Seeds:
/// - `MODULE_REPO_LINK_SEED.as_bytes()`
/// - `module_pubkey.as_ref()`
/// - `repo_pubkey.as_ref()`
pub fn module_repo_link_pda(
    program_id: &Pubkey,
    module_pubkey: &Pubkey,
    repo_pubkey: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            MODULE_REPO_LINK_SEED.as_bytes(),
            module_pubkey.as_ref(),
            repo_pubkey.as_ref(),
        ],
        program_id,
    )
}

pub fn module_repo_link_seeds<'a>(
    module_pubkey: &Pubkey,
    repo_pubkey: &Pubkey,
    bump: u8,
) -> SeedSlice<'a> {
    &[
        MODULE_REPO_LINK_SEED.as_bytes(),
        module_pubkey.as_ref(),
        repo_pubkey.as_ref(),
        &[bump],
    ]
}

// ---------------------------------------------------------------------------
// Convenience: Generic PDA Assertion
// ---------------------------------------------------------------------------

/// A small helper that verifies that an account matches the PDA derived from
/// the provided seeds and program id.
///
/// This can be useful in tests or custom validation logic outside the Anchor
/// `#[account]` constraint system.
pub fn assert_pda(
    account_key: &Pubkey,
    seeds: &[&[u8]],
    program_id: &Pubkey,
) -> Result<u8> {
    let (expected, bump) = Pubkey::find_program_address(seeds, program_id);
    require_keys_eq!(*account_key, expected, crate::errors::Unit09Error::InvalidPda);
    Ok(bump)
}
