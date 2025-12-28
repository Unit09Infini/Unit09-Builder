//! ===========================================================================
//! Unit09 – Common Validation Helpers
//! Path: contracts/unit09-program/programs/unit09_program/src/utils/validators.rs
//!
//! This module provides shared validation helpers used across instructions
//! and state implementations in the Unit09 program.
//!
//! Goals:
//! - Avoid repeating low-level checks (length, non-zero, ranges)
//! - Keep error mapping consistent (`Unit09Error`)
//! - Provide small, composable helpers that are easy to audit
//!
//! These helpers are intentionally small and explicit. They are not meant to
//! replace business logic, only to capture common guard patterns.
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;

/// Validate that a string is not empty.
pub fn assert_non_empty_str(value: &str) -> Result<()> {
    require!(!value.is_empty(), Unit09Error::StringEmpty);
    Ok(())
}

/// Validate that a string length is within a maximum bound (inclusive).
///
/// `max_len` is expressed in bytes (as returned by `len()`).
pub fn assert_max_len(value: &str, max_len: usize) -> Result<()> {
    require!(value.len() <= max_len, Unit09Error::StringTooLong);
    Ok(())
}

/// Validate that an optional string, when present, is not empty and not
/// longer than `max_len`.
pub fn assert_optional_str_len(value: &Option<String>, max_len: usize) -> Result<()> {
    if let Some(ref s) = value {
        assert_non_empty_str(s)?;
        assert_max_len(s, max_len)?;
    }
    Ok(())
}

/// Validate that a numeric value is not zero.
pub fn assert_non_zero_u64(value: u64) -> Result<()> {
    require!(value != 0, Unit09Error::ValueOutOfRange);
    Ok(())
}

/// Validate that a numeric value is not zero (u32 version).
pub fn assert_non_zero_u32(value: u32) -> Result<()> {
    require!(value != 0, Unit09Error::ValueOutOfRange);
    Ok(())
}

/// Validate a basis-points value (0–10_000).
pub fn assert_fee_bps_in_range(fee_bps: u16) -> Result<()> {
    require!(fee_bps <= MAX_FEE_BPS, Unit09Error::InvalidFeeBps);
    Ok(())
}

/// Validate that `value` does not exceed `max`.
pub fn assert_not_greater_u64(value: u64, max: u64) -> Result<()> {
    require!(value <= max, Unit09Error::ValueOutOfRange);
    Ok(())
}

/// Validate that `value` does not exceed `max`.
pub fn assert_not_greater_u32(value: u32, max: u32) -> Result<()> {
    require!(value <= max, Unit09Error::ValueOutOfRange);
    Ok(())
}

/// Basic URL prefix check. This is intentionally shallow and designed only
/// to prevent obviously malformed values from being committed on-chain.
///
/// Accepted prefixes:
/// - http://
/// - https://
/// - ipfs://
/// - ar://
pub fn assert_url_like(value: &str) -> Result<()> {
    if value.is_empty() {
        return Ok(());
    }

    let ok = value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with("ipfs://")
        || value.starts_with("ar://");

    require!(ok, Unit09Error::MetadataInvalid);
    Ok(())
}

/// Basic HTTPS-only check (used when cleartext HTTP should not be accepted).
pub fn assert_https_url(value: &str) -> Result<()> {
    if value.is_empty() {
        return Ok(());
    }

    let ok = value.starts_with("https://");
    require!(ok, Unit09Error::MetadataInvalid);
    Ok(())
}

/// Validate a semantic version tuple `(major, minor, patch)`
///
/// At least one component must be non-zero.
pub fn assert_semver_non_zero(version: (u16, u16, u16)) -> Result<()> {
    let (major, minor, patch) = version;
    require!(
        major != 0 || minor != 0 || patch != 0,
        Unit09Error::ValueOutOfRange
    );
    Ok(())
}

/// Ensure that the provided signer key matches the expected admin key.
pub fn assert_admin_signer(admin_account: &Pubkey, signer: &Pubkey) -> Result<()> {
    require_keys_eq!(*admin_account, *signer, Unit09Error::InvalidAdmin);
    Ok(())
}

/// Ensure that the provided signer key matches an expected authority key.
pub fn assert_authority_signer(authority: &Pubkey, signer: &Pubkey) -> Result<()> {
    require_keys_eq!(*authority, *signer, Unit09Error::InvalidAuthority);
    Ok(())
}

/// Ensure that a Boolean flag is true, mapping failures to a specific error.
pub fn assert_flag_true(flag: bool, err: Unit09Error) -> Result<()> {
    require!(flag, err);
    Ok(())
}

/// Ensure that a Boolean flag is false, mapping failures to a specific error.
pub fn assert_flag_false(flag: bool, err: Unit09Error) -> Result<()> {
    require!(!flag, err);
    Ok(())
}

/// Validate a tag string against a maximum length and a maximum approximate
/// number of tags separated by commas.
///
/// This is a light heuristic to keep tags compact while allowing free-form
/// usage. It does not enforce strict formatting.
pub fn assert_tags_reasonable(tags: &str, max_len: usize, max_tags: usize) -> Result<()> {
    if tags.is_empty() {
        return Ok(());
    }

    assert_max_len(tags, max_len)?;

    let count = tags.split(',').filter(|s| !s.trim().is_empty()).count();
    require!(count <= max_tags, Unit09Error::ValueOutOfRange);

    Ok(())
}

/// Validate a revision string (commit hash or label) with a maximum length.
pub fn assert_revision_len(revision: &str, max_len: usize) -> Result<()> {
    assert_max_len(revision, max_len)
}

/// Validate an observation note string against a maximum length.
pub fn assert_observation_note_len(note: &str, max_len: usize) -> Result<()> {
    assert_max_len(note, max_len)
}

/// Ensure that a deployment is marked active.
///
/// This is a small helper used in places where `Config::assert_active`
/// cannot be called directly (for example, in tests or in non-Anchor logic).
pub fn assert_deployment_active(is_active: bool) -> Result<()> {
    require!(is_active, Unit09Error::DeploymentInactive);
    Ok(())
}
