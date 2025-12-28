//! ===========================================================================
//! Unit09 – Global Metrics State
//! Path: contracts/unit09-program/programs/unit09_program/src/state/metrics.rs
//!
//! The `Metrics` account stores deployment-wide aggregate counters for the
//! Unit09 protocol. While `Repo` and `Module` keep per-entity statistics,
//! `Metrics` provides a single place to inspect global activity:
//!
//! - how many repositories are tracked
//! - how many modules and forks exist
//! - how many observation runs have occurred
//! - approximate aggregate lines of code and files processed
//!
//! This account is intentionally simple and numeric to keep read costs low
//! and make it easy for dashboards, explorers, and monitoring systems to
//! query global health at a glance.
//!
//! Typical usage:
//! - Increment counters in instruction handlers when:
//!     * a new repo/module/fork is created
//!     * an observation is recorded
//! - Use `adjust_*` methods only when reconciling counts with off-chain data.
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;

/// Global aggregate metrics for a Unit09 deployment.
///
/// This account is expected to be a PDA derived from `METRICS_SEED` and the
/// program ID. It does not store detailed per-entity metrics; those belong
/// to `Repo`, `Module`, and other state accounts.
#[account]
pub struct Metrics {
    /// Total number of repositories tracked by this deployment.
    pub total_repos: u64,

    /// Total number of modules registered across all repositories.
    pub total_modules: u64,

    /// Total number of forks created.
    pub total_forks: u64,

    /// Total number of observation runs recorded.
    pub total_observations: u64,

    /// Approximate total lines of code processed across all observations.
    pub total_lines_of_code: u64,

    /// Approximate total files processed across all observations.
    pub total_files_processed: u64,

    /// Unix timestamp of the last recorded observation.
    pub last_observation_at: i64,

    /// Unix timestamp when this metrics account was created.
    pub created_at: i64,

    /// Unix timestamp when this metrics account was last updated.
    pub updated_at: i64,

    /// Schema version for this metrics layout.
    pub schema_version: u8,

    /// Bump used for PDA derivation.
    pub bump: u8,

    /// Reserved bytes for future upgrades.
    pub reserved: [u8; 78],
}

impl Metrics {
    /// Discriminator length used by Anchor.
    pub const DISCRIMINATOR_LEN: usize = 8;

    /// Total serialized length for the `Metrics` account.
    pub const LEN: usize = Self::DISCRIMINATOR_LEN
        + 8  // total_repos: u64
        + 8  // total_modules: u64
        + 8  // total_forks: u64
        + 8  // total_observations: u64
        + 8  // total_lines_of_code: u64
        + 8  // total_files_processed: u64
        + 8  // last_observation_at: i64
        + 8  // created_at: i64
        + 8  // updated_at: i64
        + 1  // schema_version: u8
        + 1  // bump: u8
        + 78; // reserved: [u8; 78]

    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    /// Initialize the global metrics account with zeroed counters.
    ///
    /// This is typically called once from the `initialize` instruction.
    pub fn init(&mut self, bump: u8, clock: &Clock) -> Result<()> {
        let now = clock.unix_timestamp;

        self.total_repos = 0;
        self.total_modules = 0;
        self.total_forks = 0;
        self.total_observations = 0;
        self.total_lines_of_code = 0;
        self.total_files_processed = 0;
        self.last_observation_at = 0;
        self.created_at = now;
        self.updated_at = now;
        self.schema_version = CURRENT_SCHEMA_VERSION;
        self.bump = bump;
        self.reserved = [0u8; 78];

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Creation Counters
    // -----------------------------------------------------------------------

    /// Increment total repositories counter.
    pub fn increment_repos(&mut self) -> Result<()> {
        self.total_repos = self
            .total_repos
            .checked_add(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    /// Decrement total repositories counter, if you ever add repository
    /// deletion or archival.
    pub fn decrement_repos(&mut self) -> Result<()> {
        self.total_repos = self
            .total_repos
            .checked_sub(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    /// Increment total modules counter.
    pub fn increment_modules(&mut self) -> Result<()> {
        self.total_modules = self
            .total_modules
            .checked_add(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    /// Decrement total modules counter.
    pub fn decrement_modules(&mut self) -> Result<()> {
        self.total_modules = self
            .total_modules
            .checked_sub(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    /// Increment total forks counter.
    pub fn increment_forks(&mut self) -> Result<()> {
        self.total_forks = self
            .total_forks
            .checked_add(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    /// Decrement total forks counter.
    pub fn decrement_forks(&mut self) -> Result<()> {
        self.total_forks = self
            .total_forks
            .checked_sub(1)
            .ok_or(Unit09Error::CounterOverflow)?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Observation Aggregation
    // -----------------------------------------------------------------------

    /// Record a single observation and aggregate its contribution.
    ///
    /// This should be called from the `record_observation` instruction, after
    /// the per-repository update has been performed.
    pub fn record_observation(
        &mut self,
        lines_of_code: u64,
        files_processed: u32,
        clock: &Clock,
    ) -> Result<()> {
        // Bounds check using constants.
        if lines_of_code > MAX_LOC_PER_OBSERVATION {
            return err!(Unit09Error::ObservationDataTooLarge);
        }
        if files_processed as u64 > MAX_FILES_PER_OBSERVATION as u64 {
            return err!(Unit09Error::ObservationDataTooLarge);
        }

        // Increment observation count.
        self.total_observations = self
            .total_observations
            .checked_add(1)
            .ok_or(Unit09Error::CounterOverflow)?;

        // Aggregate lines of code.
        self.total_lines_of_code = self
            .total_lines_of_code
            .checked_add(lines_of_code)
            .ok_or(Unit09Error::CounterOverflow)?;

        // Aggregate files processed.
        self.total_files_processed = self
            .total_files_processed
            .checked_add(files_processed as u64)
            .ok_or(Unit09Error::CounterOverflow)?;

        // Update last observation timestamp.
        self.last_observation_at = clock.unix_timestamp;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Bulk Adjustment (Reconciliation)
    // -----------------------------------------------------------------------

    /// Adjust all metrics in one call, for example when reconciling with
    /// off-chain analytics or performing corrective actions.
    ///
    /// This is considered an advanced operation and should only be exposed
    /// to trusted admin flows.
    pub fn adjust_aggregate(
        &mut self,
        new_total_repos: Option<u64>,
        new_total_modules: Option<u64>,
        new_total_forks: Option<u64>,
        new_total_observations: Option<u64>,
        new_total_lines_of_code: Option<u64>,
        new_total_files_processed: Option<u64>,
        clock: &Clock,
    ) -> Result<()> {
        if let Some(v) = new_total_repos {
            self.total_repos = v;
        }
        if let Some(v) = new_total_modules {
            self.total_modules = v;
        }
        if let Some(v) = new_total_forks {
            self.total_forks = v;
        }
        if let Some(v) = new_total_observations {
            self.total_observations = v;
        }
        if let Some(v) = new_total_lines_of_code {
            self.total_lines_of_code = v;
        }
        if let Some(v) = new_total_files_processed {
            self.total_files_processed = v;
        }

        self.updated_at = clock.unix_timestamp;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Utility Helpers
    // -----------------------------------------------------------------------

    /// Returns a simple summary struct useful for off-chain consumers.
    ///
    /// This method is not used directly on-chain, but if you share this crate
    /// with off-chain tooling, it can be a convenient helper.
    pub fn summary(&self) -> MetricsSummary {
        MetricsSummary {
            total_repos: self.total_repos,
            total_modules: self.total_modules,
            total_forks: self.total_forks,
            total_observations: self.total_observations,
            total_lines_of_code: self.total_lines_of_code,
            total_files_processed: self.total_files_processed,
            last_observation_at: self.last_observation_at,
        }
    }
}

/// Lightweight metrics snapshot for off-chain tools.
///
/// This is not stored on-chain; it is purely a helper structure returned by
/// the `summary` method above.
#[derive(Debug, Clone, Copy)]
pub struct MetricsSummary {
    pub total_repos: u64,
    pub total_modules: u64,
    pub total_forks: u64,
    pub total_observations: u64,
    pub total_lines_of_code: u64,
    pub total_files_processed: u64,
    pub last_observation_at: i64,
}
