//! ===========================================================================
//! Unit09 – Module State
//! Path: contracts/unit09-program/programs/unit09_program/src/state/module.rs
//!
//! A `Module` represents a runnable, reusable unit of logic that Unit09
//! extracts from real-world code. Modules are:
//! - linked to a repository
//! - owned by an authority
//! - versioned
//! - tagged and categorized for discovery
//!
//! Typical lifecycle:
//! 1. A repository is registered
//! 2. Off-chain workers analyze the codebase and produce candidate modules
//! 3. Each module is registered on-chain with metadata
//! 4. Modules are updated, deprecated, or composed into forks
//!
//! This file defines:
//! - `Module` account structure
//! - size constants for rent-exempt allocation
//! - helper methods for authority checks, activation checks,
//!   usage tracking, and metadata validation
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;

/// Module account tracked by Unit09.
///
/// A module is a logical, runnable unit extracted from a repository.
/// Examples:
/// - a Solana program entry module
/// - a reusable instruction set
/// - a library or common abstraction used across programs
#[account]
pub struct Module {
    /// Arbitrary key chosen to identify this module at PDA derivation time.
    ///
    /// In most cases this will be derived from:
    /// - a hash of the module path
    /// - a logical identifier assigned by off-chain analysis
    pub module_key: Pubkey,

    /// PDA of the repository this module is associated with.
    pub repo: Pubkey,

    /// Authority that controls this module.
    ///
    /// Only this key is allowed to:
    /// - update module metadata
    /// - toggle active / deprecated status
    /// - perform module-specific administrative actions
    pub authority: Pubkey,

    /// Human-readable name for the module.
    ///
    /// Example: "unit09-router", "metrics-indexer"
    pub name: String,

    /// Off-chain metadata URI (JSON manifest).
    ///
    /// Example: Arweave, IPFS, or HTTPS location describing:
    /// - technical details
    /// - example usage
    /// - security considerations
    pub metadata_uri: String,

    /// Category classification for this module.
    ///
    /// Example values:
    /// - "program"
    /// - "library"
    /// - "indexer"
    /// - "worker"
    pub category: String,

    /// Tags for search and discovery.
    ///
    /// Example: "solana,anchor,token,module"
    pub tags: String,

    /// Whether this module is currently active.
    ///
    /// Inactive modules should not be used by default in new flows.
    pub is_active: bool,

    /// Whether this module has been deprecated.
    ///
    /// Deprecated modules remain available for historical reasons but
    /// should not be used in new designs.
    pub is_deprecated: bool,

    /// Semantic version: major component.
    ///
    /// Increment for breaking changes.
    pub major_version: u16,

    /// Semantic version: minor component.
    ///
    /// Increment for backwards-compatible feature additions.
    pub minor_version: u16,

    /// Semantic version: patch component.
    ///
    /// Increment for backwards-compatible bug fixes.
    pub patch_version: u16,

    /// How many times this module has been used or referenced by other
    /// on-chain entities (forks, compositions, deployments).
    pub usage_count: u64,

    /// Last time this module was used in a tracked way.
    pub last_used_at: i64,

    /// Creation timestamp (Unix seconds).
    pub created_at: i64,

    /// Last update timestamp (Unix seconds).
    pub updated_at: i64,

    /// Schema version for this module layout.
    pub schema_version: u8,

    /// Bump used for PDA derivation.
    pub bump: u8,

    /// Reserved space for future upgrades.
    ///
    /// This allows adding new fields later without breaking the account size.
    pub reserved: [u8; 54],
}

impl Module {
    /// Discriminator length used by Anchor.
    pub const DISCRIMINATOR_LEN: usize = 8;

    /// Maximum length in bytes (UTF-8) for the `name` field.
    pub const MAX_NAME_LEN: usize = MAX_NAME_LEN;

    /// Maximum length in bytes (UTF-8) for the `metadata_uri` field.
    pub const MAX_METADATA_URI_LEN: usize = MAX_METADATA_URI_LEN;

    /// Maximum length in bytes (UTF-8) for the `category` field.
    pub const MAX_CATEGORY_LEN: usize = MAX_MODULE_CATEGORY_LEN;

    /// Maximum length in bytes (UTF-8) for the `tags` field.
    pub const MAX_TAGS_LEN: usize = MAX_TAGS_LEN;

    /// Total serialized length of the `Module` account.
    ///
    /// Strings are encoded as:
    ///     4-byte length prefix + bytes
    pub const LEN: usize = Self::DISCRIMINATOR_LEN
        + 32 // module_key: Pubkey
        + 32 // repo: Pubkey
        + 32 // authority: Pubkey
        + 4 + Self::MAX_NAME_LEN // name: String
        + 4 + Self::MAX_METADATA_URI_LEN // metadata_uri: String
        + 4 + Self::MAX_CATEGORY_LEN // category: String
        + 4 + Self::MAX_TAGS_LEN // tags: String
        + 1 // is_active: bool
        + 1 // is_deprecated: bool
        + 2 // major_version: u16
        + 2 // minor_version: u16
        + 2 // patch_version: u16
        + 8 // usage_count: u64
        + 8 // last_used_at: i64
        + 8 // created_at: i64
        + 8 // updated_at: i64
        + 1 // schema_version: u8
        + 1 // bump: u8
        + 54; // reserved: [u8; 54]

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    /// Initialize a new module.
    ///
    /// This is typically called from the `register_module` instruction.
    pub fn init(
        &mut self,
        module_key: Pubkey,
        repo: Pubkey,
        authority: Pubkey,
        name: String,
        metadata_uri: String,
        category: String,
        tags: String,
        version: (u16, u16, u16),
        bump: u8,
        clock: &Clock,
    ) -> Result<()> {
        Self::validate_name(&name)?;
        Self::validate_metadata_uri(&metadata_uri)?;
        Self::validate_category(&category)?;
        Self::validate_tags(&tags)?;
        Self::validate_version(version)?;

        let (major, minor, patch) = version;

        self.module_key = module_key;
        self.repo = repo;
        self.authority = authority;
        self.name = name;
        self.metadata_uri = metadata_uri;
        self.category = category;
        self.tags = tags;
        self.is_active = true;
        self.is_deprecated = false;
        self.major_version = major;
        self.minor_version = minor;
        self.patch_version = patch;
        self.usage_count = 0;
        self.last_used_at = 0;
        self.created_at = clock.unix_timestamp;
        self.updated_at = clock.unix_timestamp;
        self.schema_version = CURRENT_SCHEMA_VERSION;
        self.bump = bump;
        self.reserved = [0u8; 54];

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Metadata / Version Updates
    // -----------------------------------------------------------------------

    /// Apply updates to the module metadata and status.
    ///
    /// Used by `update_module` or similar instructions to mutate fields
    /// without reconstructing the full struct.
    pub fn apply_update(
        &mut self,
        maybe_name: Option<String>,
        maybe_metadata_uri: Option<String>,
        maybe_category: Option<String>,
        maybe_tags: Option<String>,
        maybe_is_active: Option<bool>,
        maybe_is_deprecated: Option<bool>,
        maybe_version: Option<(u16, u16, u16)>,
        clock: &Clock,
    ) -> Result<()> {
        if let Some(name) = maybe_name {
            Self::validate_name(&name)?;
            self.name = name;
        }

        if let Some(metadata_uri) = maybe_metadata_uri {
            Self::validate_metadata_uri(&metadata_uri)?;
            self.metadata_uri = metadata_uri;
        }

        if let Some(category) = maybe_category {
            Self::validate_category(&category)?;
            self.category = category;
        }

        if let Some(tags) = maybe_tags {
            Self::validate_tags(&tags)?;
            self.tags = tags;
        }

        if let Some(is_active) = maybe_is_active {
            self.is_active = is_active;
        }

        if let Some(is_deprecated) = maybe_is_deprecated {
            self.is_deprecated = is_deprecated;
        }

        if let Some(version) = maybe_version {
            Self::validate_version(version)?;
            let (major, minor, patch) = version;
            self.major_version = major;
            self.minor_version = minor;
            self.patch_version = patch;
        }

        self.updated_at = clock.unix_timestamp;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Authority and Activation Guards
    // -----------------------------------------------------------------------

    /// Ensure that the signer is the authority of this module.
    pub fn assert_authority(&self, signer: &Signer) -> Result<()> {
        if signer.key() != self.authority {
            return err!(Unit09Error::InvalidAuthority);
        }
        Ok(())
    }

    /// Ensure that the module is currently active.
    pub fn assert_active(&self) -> Result<()> {
        if !self.is_active {
            return err!(Unit09Error::ModuleInactive);
        }
        Ok(())
    }

    /// Ensure that the module is not deprecated.
    pub fn assert_not_deprecated(&self) -> Result<()> {
        if self.is_deprecated {
            return err!(Unit09Error::ModuleImmutable);
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Usage Tracking
    // -----------------------------------------------------------------------

    /// Record a usage event for this module.
    ///
    /// This is expected to be called by instructions or off-chain actors
    /// whenever the module is used in a meaningful way (for example when
    /// building or executing a composed system).
    pub fn record_usage(&mut self, clock: &Clock) -> Result<()> {
        self.usage_count = self
            .usage_count
            .checked_add(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        self.last_used_at = clock.unix_timestamp;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Validation Helpers
    // -----------------------------------------------------------------------

    /// Validate the module name.
    fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if name.len() > Self::MAX_NAME_LEN {
            return err!(Unit09Error::StringTooLong);
        }
        Ok(())
    }

    /// Validate the metadata URI.
    fn validate_metadata_uri(uri: &str) -> Result<()> {
        if uri.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if uri.len() > Self::MAX_METADATA_URI_LEN {
            return err!(Unit09Error::StringTooLong);
        }

        // Very basic structural check; does not attempt full URL validation.
        let has_known_prefix = uri.starts_with("http://")
            || uri.starts_with("https://")
            || uri.starts_with("ipfs://")
            || uri.starts_with("ar://");

        if !has_known_prefix {
            // Not strictly an error for all deployments, but this helps
            // keep metadata consistent in early versions.
            return err!(Unit09Error::MetadataInvalid);
        }

        Ok(())
    }

    /// Validate the module category.
    fn validate_category(category: &str) -> Result<()> {
        if category.is_empty() {
            return err!(Unit09Error::StringEmpty);
        }
        if category.len() > Self::MAX_CATEGORY_LEN {
            return err!(Unit09Error::StringTooLong);
        }
        Ok(())
    }

    /// Validate the tags string.
    fn validate_tags(tags: &str) -> Result<()> {
        if tags.len() > Self::MAX_TAGS_LEN {
            return err!(Unit09Error::StringTooLong);
        }
        Ok(())
    }

    /// Validate semantic version components.
    fn validate_version(version: (u16, u16, u16)) -> Result<()> {
        let (major, minor, patch) = version;

        // Basic sanity checks; you can enforce more complex rules off-chain.
        if major == 0 && minor == 0 && patch == 0 {
            // All-zero version is usually undesired.
            return err!(Unit09Error::ValueOutOfRange);
        }

        // No upper bounds enforcement here; u16 is sufficient.
        Ok(())
    }
}
