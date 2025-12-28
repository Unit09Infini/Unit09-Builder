//! ===========================================================================
//! Unit09 – Events
//! Path: contracts/unit09-program/programs/unit09_program/src/events.rs
//!
//! This module defines all on-chain events emitted by the Unit09 program.
//!
//! Goals:
//! - Every meaningful state transition emits a structured event
//! - Indexers, dashboards, and analytics pipelines can reconstruct:
//!     * configuration history
//!     * repository and module lifecycles
//!     * forks (Unit09 variants) and their states
//!     * observation runs and aggregate metrics
//!     * global metadata evolution
//! - Events are stable and versioned through schema evolution
//!
//! Notes:
//! - The core events (ConfigUpdated, RepoRegistered, RepoUpdated,
//!   ModuleRegistered, ModuleUpdated, ForkCreated, ForkStateUpdated,
//!   ObservationRecorded, MetricsUpdated) are used directly by the
//!   instruction handlers already provided. Their field layout MUST remain
//!   compatible with those handlers to compile correctly.
//!
//! - Additional events are defined for richer telemetry and future use,
//!   but they are not yet wired into all handlers. They are provided here
//!   so you can easily emit them later as the protocol grows.
//!
//! ===========================================================================

use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Core Configuration Events
// ---------------------------------------------------------------------------

/// Emitted whenever the global configuration is created or updated.
///
/// This event is critical for reconstructing the fee and limit configuration
/// over time, especially when analyzing behavior across different epochs
/// of the protocol.
#[event]
pub struct ConfigUpdated {
    /// Admin authority for this deployment.
    pub admin: Pubkey,
    /// Current fee in basis points (0–10_000).
    pub fee_bps: u16,
    /// Maximum number of modules allowed per repository.
    pub max_modules_per_repo: u32,
}

/// Emitted when a new configuration admin is explicitly rotated.
///
/// This is not wired into the base handlers yet, but can be used if you
/// introduce an explicit admin rotation instruction.
#[event]
pub struct AdminRotated {
    /// Previous admin authority.
    pub old_admin: Pubkey,
    /// New admin authority.
    pub new_admin: Pubkey,
    /// Unix timestamp of the rotation.
    pub rotated_at: i64,
}

// ---------------------------------------------------------------------------
// Repository Events
// ---------------------------------------------------------------------------

/// Emitted when a repository is first registered with Unit09.
///
/// A repository is a logical representation of a real-world codebase that
/// Unit09 will observe and modularize.
#[event]
pub struct RepoRegistered {
    /// PDA of the repository account.
    pub repo: Pubkey,
    /// Authority (owner) that controls this repository entry.
    pub owner: Pubkey,
    /// URL where the repository can be accessed (GitHub, GitLab, etc.).
    pub url: String,
}

/// Emitted when repository metadata is updated.
///
/// The base implementation uses this event when the URL is changed. You can
/// extend the instruction handler to emit this for other metadata updates
/// as well.
#[event]
pub struct RepoUpdated {
    /// PDA of the repository account.
    pub repo: Pubkey,
    /// New URL after the update.
    pub url: String,
}

/// Emitted when a repository is activated or deactivated.
///
/// This is useful for dashboards and workers to stop or start observation
/// runs against a given repository.
#[event]
pub struct RepoActivationChanged {
    /// PDA of the repository account.
    pub repo: Pubkey,
    /// Whether the repository is now active.
    pub is_active: bool,
    /// Unix timestamp of the change.
    pub updated_at: i64,
}

// ---------------------------------------------------------------------------
// Module Events
// ---------------------------------------------------------------------------

/// Emitted when a new module is registered for a repository.
///
/// A module represents a runnable, reusable unit produced by Unit09’s
/// analysis of real-world code.
#[event]
pub struct ModuleRegistered {
    /// PDA of the module account.
    pub module: Pubkey,
    /// PDA of the repository this module belongs to.
    pub repo: Pubkey,
    /// Authority (owner) that controls this module.
    pub authority: Pubkey,
    /// Human-readable name of the module.
    pub name: String,
    /// Version number assigned at registration time.
    pub version: u32,
}

/// Emitted when a module is updated.
///
/// This typically reflects a change in:
/// - version number
/// - metadata URI
/// - active/inactive status
#[event]
pub struct ModuleUpdated {
    /// PDA of the module account.
    pub module: Pubkey,
    /// New version number after the update.
    pub version: u32,
}

/// Emitted when a module is linked to a repository or relinked from one
/// repository to another.
#[event]
pub struct ModuleLinkedToRepo {
    /// PDA of the module account.
    pub module: Pubkey,
    /// PDA of the repository the module is linked to.
    pub repo: Pubkey,
    /// Unix timestamp of the link operation.
    pub linked_at: i64,
}

/// Emitted when a module is explicitly marked as active or inactive.
///
/// This event is not currently wired into the provided handler, but you can
/// emit it inside `update_module` once you treat `is_active` toggles.
#[event]
pub struct ModuleActivationChanged {
    /// PDA of the module account.
    pub module: Pubkey,
    /// Whether the module is now active.
    pub is_active: bool,
    /// Unix timestamp of the change.
    pub updated_at: i64,
}

// ---------------------------------------------------------------------------
// Module Version Events (optional, for version history tracking)
// ---------------------------------------------------------------------------

/// Emitted when a new module version entry is created.
///
/// If you track `ModuleVersion` accounts, this helps indexers reconstruct
/// a full lineage of versions.
#[event]
pub struct ModuleVersionCreated {
    /// PDA of the parent module.
    pub module: Pubkey,
    /// Version number that was created.
    pub version: u32,
    /// Metadata URI associated with this version.
    pub metadata_uri: String,
    /// Unix timestamp of the creation time.
    pub created_at: i64,
}

// ---------------------------------------------------------------------------
// Fork Events (Unit09 Variants)
// ---------------------------------------------------------------------------

/// Emitted when a new fork (Unit09 variant) is created.
///
/// Forks can represent different personalities, configurations, or module
/// selections for the Unit09 AI raccoon.
#[event]
pub struct ForkCreated {
    /// PDA of the fork account.
    pub fork: Pubkey,
    /// Parent reference (often another fork or a root identity).
    pub parent: Pubkey,
    /// Owner authority controlling this fork.
    pub owner: Pubkey,
    /// Human-readable label describing the fork.
    pub label: String,
}

/// Emitted when the active state of a fork is toggled or when important
/// status changes occur.
#[event]
pub struct ForkStateUpdated {
    /// PDA of the fork account.
    pub fork: Pubkey,
    /// Whether the fork is currently active.
    pub active: bool,
}

/// Emitted when the owner of a fork is rotated.
///
/// This is useful when transferring control of a fork to a new operator.
#[event]
pub struct ForkOwnerChanged {
    /// PDA of the fork account.
    pub fork: Pubkey,
    /// Previous owner of the fork.
    pub old_owner: Pubkey,
    /// New owner of the fork.
    pub new_owner: Pubkey,
    /// Unix timestamp of the ownership change.
    pub changed_at: i64,
}

// ---------------------------------------------------------------------------
// Observation and Metrics Events
// ---------------------------------------------------------------------------

/// Emitted when an observation run over a repository is recorded.
///
/// This event allows indexers and dashboards to reconstruct how much
/// code has been analyzed over time and how frequently Unit09’s workers
/// are observing a repository.
#[event]
pub struct ObservationRecorded {
    /// PDA of the repository that was observed.
    pub repo: Pubkey,
    /// Slot at which the observation was recorded.
    pub slot: u64,
    /// Number of lines of code processed in this observation run.
    pub lines_of_code: u64,
    /// Number of files processed in this observation run.
    pub files_processed: u32,
}

/// Emitted when aggregate metrics are updated in bulk.
///
/// This event is intended to reflect large-scale corrections or alignment
/// with off-chain analytics and may not be emitted on every observation.
#[event]
pub struct MetricsUpdated {
    /// Total repositories tracked by this deployment.
    pub total_repos: u64,
    /// Total modules registered across all repositories.
    pub total_modules: u64,
    /// Total forks created.
    pub total_forks: u64,
    /// Total observation runs recorded.
    pub total_observations: u64,
}

/// Emitted when a soft or hard limit for metrics has been reached
/// and certain actions may be throttled or restricted off-chain.
#[event]
pub struct MetricsLimitReached {
    /// Optional identifier describing which limit was reached.
    pub limit_key: String,
    /// Current value at the moment the limit was observed.
    pub current_value: u64,
    /// Unix timestamp when the limit event occurred.
    pub observed_at: i64,
}

// ---------------------------------------------------------------------------
// Global Metadata and Lifecycle Events
// ---------------------------------------------------------------------------

/// Emitted when global metadata describing the Unit09 deployment is set
/// or updated.
///
/// This typically contains:
//  - human-readable description
/// - tags or keywords used by explorers and dashboards
#[event]
pub struct GlobalMetadataUpdated {
    /// Admin that performed this update.
    pub admin: Pubkey,
    /// Short description string summary (truncated for event payload).
    pub description_preview: String,
    /// Tags string summary (truncated for event payload).
    pub tags_preview: String,
    /// Unix timestamp of the update.
    pub updated_at: i64,
}

/// Emitted when the lifecycle state of the deployment changes.
///
/// This can be used if you introduce lifecycle phases such as:
/// - bootstrapping
/// - normal operation
/// - frozen / read-only
/// - migration / sunset
#[event]
pub struct LifecycleStateChanged {
    /// Optional numeric or string code representing the lifecycle state.
    pub state_code: u8,
    /// Unix timestamp of the state change.
    pub changed_at: i64,
    /// Free-form note hash or reference (for off-chain documentation).
    pub note_ref: String,
}

// ---------------------------------------------------------------------------
// Authority / Role Events (optional)
// ---------------------------------------------------------------------------

/// Emitted when a role is assigned to an authority account.
///
/// While basic authority checks can be done directly in accounts,
/// emitting events for role assignments makes governance operations
/// easier to index and audit.
#[event]
pub struct AuthorityRoleAssigned {
    /// Account receiving the role.
    pub authority: Pubkey,
    /// Role identifier (for example: "admin", "maintainer", "observer").
    pub role: String,
    /// Unix timestamp of the assignment.
    pub assigned_at: i64,
}

/// Emitted when a role is revoked from an authority account.
#[event]
pub struct AuthorityRoleRevoked {
    /// Account losing the role.
    pub authority: Pubkey,
    /// Role identifier that was revoked.
    pub role: String,
    /// Unix timestamp of the revocation.
    pub revoked_at: i64,
}

// ---------------------------------------------------------------------------
// Utility Event For Debugging (optional)
// ---------------------------------------------------------------------------

/// Generic log-style event that can be used for debugging or for lightweight
/// application-level signals without requiring a dedicated event type.
///
/// This is optional and can be removed if you prefer having only
/// strongly-typed events.
#[event]
pub struct Unit09Log {
    /// Free-form category string for the log message.
    pub category: String,
    /// Message text (keep relatively short to avoid bloating logs).
    pub message: String,
    /// Unix timestamp when the log was emitted.
    pub logged_at: i64,
}
