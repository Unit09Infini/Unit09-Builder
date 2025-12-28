//! ===================================================================================
//! Unit09 Program – On-chain AI Raccoon Core
//! Path: contracts/unit09-program/programs/unit09_program/src/lib.rs
//!
//! This crate defines the on-chain core for Unit09:
//! - A configuration layer (fees, limits, authority)
//! - Repository tracking for real-world codebases
//! - Module registry for generated runnable units
//! - Fork entities representing evolving Unit09 variants
//! - Metrics and lifecycle tracking for the protocol
//!
//! The design goal is to provide a clean, well-documented entrypoint that
//! external clients (SDKs, dashboards, workers) can integrate with while
//! keeping all business logic in dedicated instruction modules.
//!
//! ===================================================================================

use anchor_lang::prelude::*;

// Public submodules
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

// Re-export selected items for easier access by integration code
pub use crate::constants::*;
pub use crate::errors::Unit09Error;
pub use crate::events::*;
pub use crate::instructions::*;
pub use crate::state::*;
pub use crate::utils::*;

/// Program ID for Unit09.
///
/// This value is for localnet / development by default.  
/// For real deployments:
/// - Update `declare_id!` with the actual program address
/// - Update `Anchor.toml` `programs.*` sections
/// - Update `contracts/unit09-program/program-id.md`
declare_id!("UNIT9mB7Z2F8cUXa11111111111111111111111111");

/// Anchor program entrypoint.
///
/// Each exported function here forwards to a dedicated handler in the
/// `instructions` module. This keeps business logic out of the main
/// file and makes it easier to reason about, test, and evolve.
#[program]
pub mod unit09_program {
    use super::*;

    // -------------------------------------------------------------------------
    //  Initialization and Configuration
    // -------------------------------------------------------------------------

    /// Initialize global configuration and metrics for Unit09.
    ///
    /// This should be called exactly once for a given deployment. It sets:
    /// - Admin authority (who can update config and metadata)
    /// - Fee basis points
    /// - Maximum allowed modules per repository
    ///
    /// Accounts:
    /// - `config`   – PDA storing global configuration
    /// - `metrics`  – PDA storing global metrics
    /// - `payer`    – funds account creations
    /// - `system_program`
    pub fn initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
        instructions::initialize::handler(ctx, args)
    }

    /// Update configuration values such as fee basis points or module limits.
    ///
    /// Only the admin defined in the `Config` account is allowed to call this.
    ///
    /// Accounts:
    /// - `config` – existing configuration PDA
    /// - `admin`  – signer, must match `config.admin`
    pub fn set_config(ctx: Context<SetConfig>, args: SetConfigArgs) -> Result<()> {
        instructions::set_config::handler(ctx, args)
    }

    // -------------------------------------------------------------------------
    //  Repository Management
    // -------------------------------------------------------------------------

    /// Register a repository that Unit09 will observe and modularize.
    ///
    /// Example use cases:
    /// - Track an open-source Solana program
    /// - Track a monorepo that contains multiple Solana-related modules
    ///
    /// Accounts:
    /// - `repo`       – new PDA derived from repo key
    /// - `repo_key`   – arbitrary public key representing the logical repo
    /// - `metrics`    – global metrics PDA
    /// - `authority`  – signer who owns this repository entry
    /// - `system_program`
    pub fn register_repo(ctx: Context<RegisterRepo>, args: RegisterRepoArgs) -> Result<()> {
        instructions::register_repo::handler(ctx, args)
    }

    /// Update repository metadata and activation status.
    ///
    /// Allows the authority to:
    /// - Change the repository URL
    /// - Enable/disable the repository
    ///
    /// Accounts:
    /// - `repo`      – target repository account
    /// - `authority` – signer, must match `repo.authority`
    pub fn update_repo(ctx: Context<UpdateRepo>, args: UpdateRepoArgs) -> Result<()> {
        instructions::update_repo::handler(ctx, args)
    }

    // -------------------------------------------------------------------------
    //  Module Management
    // -------------------------------------------------------------------------

    /// Register a generated module for a repository.
    ///
    /// This represents a runnable building block extracted from a real codebase.
    ///
    /// Accounts:
    /// - `repo`        – repository to which the module belongs
    /// - `module_key`  – arbitrary public key used to derive module PDA
    /// - `module`      – new module PDA
    /// - `metrics`     – global metrics PDA
    /// - `authority`   – signer who owns this module
    /// - `system_program`
    pub fn register_module(ctx: Context<RegisterModule>, args: RegisterModuleArgs) -> Result<()> {
        instructions::register_module::handler(ctx, args)
    }

    /// Update an existing module.
    ///
    /// This can be used to:
    /// - Increment the version number
    /// - Change metadata URI
    /// - Soft-disable a module
    ///
    /// Accounts:
    /// - `module`    – module PDA
    /// - `authority` – signer, must match `module.authority`
    pub fn update_module(ctx: Context<UpdateModule>, args: UpdateModuleArgs) -> Result<()> {
        instructions::update_module::handler(ctx, args)
    }

    /// Link an existing module to a repository.
    ///
    /// This is useful when a module was registered first and later assigned
    /// to a repository, or when reorganizing modules across repositories.
    ///
    /// Accounts:
    /// - `module`    – module to relink (authority must sign)
    /// - `repo`      – target repository
    /// - `authority` – signer, must match `module.authority`
    pub fn link_module_to_repo(ctx: Context<LinkModuleToRepo>) -> Result<()> {
        instructions::link_module_to_repo::handler(ctx)
    }

    // -------------------------------------------------------------------------
    //  Fork Management
    // -------------------------------------------------------------------------

    /// Create a fork entity representing a new Unit09 variant.
    ///
    /// A fork can represent:
    /// - A different configuration of Unit09
    /// - A module subset
    /// - A different “personality” or evolution path
    ///
    /// Accounts:
    /// - `fork_key`   – arbitrary public key used to derive fork PDA
    /// - `fork`       – new fork PDA
    /// - `metrics`    – metrics PDA to increment fork counters
    /// - `owner`      – signer who owns this fork
    /// - `system_program`
    pub fn create_fork(ctx: Context<CreateFork>, args: CreateForkArgs) -> Result<()> {
        instructions::create_fork::handler(ctx, args)
    }

    /// Update the state of an existing fork.
    ///
    /// This can:
    /// - Activate or deactivate a fork
    /// - Update its metadata URI
    ///
    /// Accounts:
    /// - `fork`   – fork PDA
    /// - `owner`  – signer, must match `fork.owner`
    pub fn update_fork_state(ctx: Context<UpdateForkState>, args: UpdateForkStateArgs) -> Result<()> {
        instructions::update_fork_state::handler(ctx, args)
    }

    // -------------------------------------------------------------------------
    //  Observations and Metrics
    // -------------------------------------------------------------------------

    /// Record an observation run for a repository.
    ///
    /// This function is intended to be called by off-chain workers that:
    /// - Scan codebases
    /// - Count lines and files
    /// - Report back aggregated statistics
    ///
    /// Accounts:
    /// - `repo`      – repository being observed
    /// - `metrics`   – global metrics PDA
    /// - `observer`  – signer (worker, operator, or automation key)
    pub fn record_observation(ctx: Context<RecordObservation>, args: RecordObservationArgs) -> Result<()> {
        instructions::record_observation::handler(ctx, args)
    }

    /// Manually adjust aggregate metrics.
    ///
    /// This is an escape hatch for:
    /// - Correcting historical counts
    /// - Aligning with off-chain analytics
    ///
    /// Accounts:
    /// - `metrics` – metrics PDA
    /// - `updater` – signer authorized by off-chain policy (enforced off-chain)
    pub fn record_metrics(ctx: Context<RecordMetrics>, args: RecordMetricsArgs) -> Result<()> {
        instructions::record_metrics::handler(ctx, args)
    }

    // -------------------------------------------------------------------------
    //  Global Metadata
    // -------------------------------------------------------------------------

    /// Set or update human-readable metadata for the entire Unit09 deployment.
    ///
    /// This may include:
    /// - Description text
    /// - Tags used by dashboards and explorers
    ///
    /// Accounts:
    /// - `config`    – configuration PDA (admin is enforced)
    /// - `metadata`  – global metadata PDA (init if needed)
    /// - `admin`     – signer, must match `config.admin`
    /// - `system_program`
    pub fn set_metadata(ctx: Context<SetMetadata>, args: SetMetadataArgs) -> Result<()> {
        instructions::set_metadata::handler(ctx, args)
    }
}

// ===================================================================================
//  Public API Surface: State and Utilities
// ===================================================================================

/// State module re-export.
/// 
/// This pattern allows downstream crates to import `state::*` from the root:
/// `use unit09_program::state::*;`
pub mod state {
    pub mod config;
    pub mod repo;
    pub mod module;
    pub mod module_version;
    pub mod fork;
    pub mod lifecycle;
    pub mod metrics;
    pub mod authority;

    pub use config::*;
    pub use repo::*;
    pub use module::*;
    pub use module_version::*;
    pub use fork::*;
    pub use lifecycle::*;
    pub use metrics::*;
    pub use authority::*;
}

/// Utility helpers re-export.
///
/// Contains helpers for:
/// - PDA seeds
/// - Common validators
/// - Time utilities
pub mod utils {
    pub mod seeds;
    pub mod validators;
    pub mod time;

    pub use seeds::*;
    pub use validators::*;
    pub use time::*;
}

/// Instruction module re-export (already used above, but also available to
/// external crates). This is useful if external tools want to construct
/// `Context<Instruction>` types directly.
pub use instructions::*;

// ===================================================================================
//  Optional: Test-only helpers (gated by cfg)
// ===================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Basic sanity check for the program ID.
    #[test]
    fn program_id_is_non_default() {
        // This check can be updated once you have a real, non-placeholder ID.
        let id = crate::ID;
        assert_ne!(
            id.to_string(),
            "11111111111111111111111111111111",
            "Program ID must not be the default system ID"
        );
    }
}
