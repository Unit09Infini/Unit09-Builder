//! ===========================================================================
//! Unit09 – Module Version State
//! Path: contracts/unit09-program/programs/unit09_program/src/state/module_version.rs
//!
//! A `ModuleVersion` represents a concrete, immutable snapshot of a `Module` at
//! a specific semantic version.
//!
//! While `Module` stores the latest state and mutable flags, `ModuleVersion`
//! captures a time-stamped, append-only history of how the module evolved.
//!
//! Typical usage pattern:
//! 1. A `Module` is created with an initial version (major, minor, patch)
//! 2. When a new version is published, a new `ModuleVersion` account is created
//! 3. Off-chain indexers can:
//!    - reconstruct full changelog for each module
//!    - show historical metadata
//!    - diff versions for audits
//!
//! This file defines:
//! - `ModuleVersion` account structure
//! - length constants for rent-exempt allocation
//! - helpers for initialization and validation
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;

/// Immutable version snapshot for a `Module`.
///
/// Every time a module version is published, a new `ModuleVersion` account
/// can be created with the relevant metadata. The `Module` account itself
/// only stores the latest version and flags.
#[account]
pub struct ModuleVersion {
    /// PDA of the parent module.
    pub module: Pubkey,

    /// Semantic version components for this snapshot.
    ///
    /// These should match the corresponding fields in the parent `Module`
    /// at the time this version is created.
    pub major_version: u16,
    pub minor_version: u16,
    pub patch_version: u16,

    /// Off-chain metadata URI for this specific version.
    ///
    /// This may differ from the parent `Module` metadata URI when:
    /// - version-specific documentation is provided
    /// - versioned build artifacts are stored separately
    pub metadata_uri: String,

    /// Optional URI pointing to version-specific release notes or changelog.
    ///
    /// Example:
    /// - "https://unit09.org/releases/module-x/v1.2.0"
    /// - "ipfs://.../changelog.json"
    pub changelog_uri: String,

    /// Optional human-readable label or codename for this version.
    ///
    /// Example: "alpha", "beta", "rc1", or a short internal label.
    pub label: String,

    /// Whether this version is considered stable.
    ///
    /// For example:
    /// - true for releases intended for production use
    /// - false for alpha, beta, or canary builds
    pub is_stable: bool,

    /// Whether this version is marked as deprecated.
    ///
    /// This allows a module to keep historical versions while signaling
    /// which ones should no longer be used.
    pub is_deprecated: bool,

    /// Unix timestamp when this version was created.
    pub created_at: i64,

    /// Unix timestamp when this version was marked as deprecated, if at all.
    ///
    /// Zero means "not deprecated" or "timestamp not recorded".
    pub deprecated_at: i64,

    /// Authority that created this version snapshot.
    ///
    /// This will usually match the module authority, but may differ if
    /// a delegated maintainer is allowed to publish versions.
    pub created_by: Pubkey,

    /// Schema version for this account layout.
    pub schema_version: u8,

    /// Bump used for PDA derivation.
    pub bump: u8,

    /// Reserved space for future upgrades.
    pub reserved: [u8; 63],
}

impl ModuleVersion {
    /// Discriminator length used by Anchor.
    pub const DISCRIMINATOR_LEN: usize = 8;

    /// Maximum length of version-specific metadata URI.
    pub const MAX_METADATA_URI_LEN: usize = MAX_METADATA_URI_LEN;

    /// Maximum length of the changelog URI.
    pub const MAX_CHANGELOG_URI_LEN: usize = MAX_METADATA_URI_LEN;

    /// Maximum length of the label string.
    pub const MAX_LABEL_LEN: usize = MAX_NAME_LEN;

    /// Total serialized length of the `ModuleVersion` account.
    ///
    /// Strings are encoded as:
    ///     4-byte length prefix + bytes
    pub const LEN: usize = Self::DISCRIMINATOR_LEN
        + 32 // module: Pubkey
        + 2  // major_version: u16
        + 2  // minor_version: u16
        + 2  // patch_version: u16
        + 4 + Self::MAX_METADATA_URI_LEN // metadata_uri: String
        + 4 + Self::MAX_CHANGELOG_URI_LEN // changelog_uri: String
        + 4 + Self::MAX_LABEL_LEN // label: String
        + 1  // is_stable: bool
        + 1  // is_deprecated: bool
        + 8  // created_at: i64
        + 8  // deprecated_at: i64
        + 32 // created_by: Pubkey
        + 1  // schema_version: u8
        + 1  // bump: u8
        + 63; // reserved: [u8; 63]

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    /// Initialize a new immutable module version snapshot.
    ///
    /// This is typically called from a dedicated `register_module_version`
    /// instruction or integrated into module update flows.
    pub fn init(
        &mut self,
        module: Pubkey,
        created_by: Pubkey,
        version: (u16, u16, u16),
        metadata_uri: String,
        changelog_uri: String,
        label: String,
        is_stable: bool,
        bump: u8,
        clock: &Clock,
    ) -> Result<()> {
        Self::validate_version(version)?;
        Self::validate_metadata_uri(&metadata_uri)?;
        Self::validate_changelog_uri(&changelog_uri)?;
        Self::validate_label(&label)?;

        let (major, minor, patch) = version;

        self.module = module;
        self.major_version = major;
        self.minor_version = minor;
        self.patch_version = patch;
        self.metadata_uri = metadata_uri;
        self.changelog_uri = changelog_uri;
        self.label = label;
        self.is_stable = is_stable;
        self.is_deprecated = false;
        self.created_at = clock.unix_timestamp;
        self.deprecated_at = 0;
        self.created_by = created_by;
        self.schema_version = CURRENT_SCHEMA_VERSION;
        self.bump = bump;
        self.reserved = [0u8; 63];

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Deprecation Logic
    // -----------------------------------------------------------------------

    /// Mark this version as deprecated.
    ///
    /// Note: since `ModuleVersion` is conceptually immutable, this is a
    /// soft deprecation flag. In practice, immutability means:
    /// - version number and metadata URIs are not changed
    /// - only deprecation status and timestamp are updated
    pub fn deprecate(&mut self, clock: &Clock) -> Result<()> {
        if self.is_deprecated {
            return err!(Unit09Error::MigrationAlreadyApplied);
        }

        self.is_deprecated = true;
        self.deprecated_at = clock.unix_timestamp;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Helpers and Validation
    // -----------------------------------------------------------------------

    /// Check that version components are sensible.
    fn validate_version(version: (u16, u16, u16)) -> Result<()> {
        let (major, minor, patch) = version;

        // All-zero version is discouraged for published snapshots.
        if major == 0 && minor == 0 && patch == 0 {
            return err!(Unit09Error::ValueOutOfRange);
        }

        Ok(())
    }

    /// Validate metadata URI for this version.
    fn validate_metadata_uri(uri: &str) -> Result<()> {
        if uri.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if uri.len() > Self::MAX_METADATA_URI_LEN {
            return err!(Unit09Error::StringTooLong);
        }

        let has_known_prefix = uri.starts_with("http://")
            || uri.starts_with("https://")
            || uri.starts_with("ipfs://")
            || uri.starts_with("ar://");

        if !has_known_prefix {
            return err!(Unit09Error::MetadataInvalid);
        }

        Ok(())
    }

    /// Validate changelog URI.
    ///
    /// This field is allowed to be empty; in that case, it simply means
    /// no dedicated changelog has been provided.
    fn validate_changelog_uri(uri: &str) -> Result<()> {
        if uri.is_empty() {
            // Empty is allowed.
            return Ok(());
        }
        if uri.len() > Self::MAX_CHANGELOG_URI_LEN {
            return err!(Unit09Error::StringTooLong);
        }

        let has_known_prefix = uri.starts_with("http://")
            || uri.starts_with("https://")
            || uri.starts_with("ipfs://")
            || uri.starts_with("ar://");

        if !has_known_prefix {
            return err!(Unit09Error::MetadataInvalid);
        }

        Ok(())
    }

    /// Validate version label.
    fn validate_label(label: &str) -> Result<()> {
        if label.len() > Self::MAX_LABEL_LEN {
            return err!(Unit09Error::StringTooLong);
        }
        Ok(())
    }
}
