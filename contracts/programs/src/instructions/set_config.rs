//! ===========================================================================
//! Unit09 – Set Config Instruction
//! Path: contracts/unit09-program/programs/unit09_program/src/instructions/set_config.rs
//!
//! This instruction updates the global configuration of a Unit09 deployment.
//!
//! It modifies the `Config` singleton account, allowing an authorized admin to:
//! - adjust protocol-wide fee basis points
//! - change the maximum modules-per-repository limit
//! - toggle the active flag
//! - update an off-chain policy reference hash
//!
//! Notes:
//! - Only the current `Config::admin` is allowed to call this instruction.
//! - All fields are optional; only provided values are updated.
//! - Bounds and validity checks are delegated to `Config::apply_update`.
//! - A `ConfigUpdated` event is emitted for indexers and dashboards.
//!
//! ===========================================================================

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::Unit09Error;
use crate::events::ConfigUpdated;
use crate::state::Config;

/// Arguments for the `set_config` instruction.
///
/// All fields are optional. If a field is `None`, the corresponding value
/// on the `Config` account is left unchanged.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SetConfigArgs {
    /// Optional new fee in basis points (0–10_000).
    ///
    /// If `Some`, the value is validated against `MAX_FEE_BPS` and then
    /// applied to `Config::fee_bps`.
    pub fee_bps: Option<u16>,

    /// Optional new maximum modules-per-repository value.
    ///
    /// If `Some`, the value must be non-zero.
    pub max_modules_per_repo: Option<u32>,

    /// Optional new active flag.
    ///
    /// If `Some(false)`, the deployment can be marked inactive. Handlers
    /// that call `Config::assert_active` will start failing after this.
    pub is_active: Option<bool>,

    /// Optional new policy reference (hash or opaque bytes).
    ///
    /// If not provided, the existing policy reference is left unchanged.
    pub policy_ref: Option<[u8; 32]>,
}

/// Accounts required for the `set_config` instruction.
#[derive(Accounts)]
pub struct SetConfig<'info> {
    /// Admin signer that is authorized to modify the global configuration.
    ///
    /// Must match `config.admin`.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// Global configuration account.
    ///
    /// PDA:
    ///   seeds = [CONFIG_SEED.as_bytes()]
    ///   bump = config.bump
    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    /// System program (required by Anchor for CPI safety in some flows).
    pub system_program: Program<'info, System>,

    /// Clock sysvar used for timestamps.
    pub clock: Sysvar<'info, Clock>,
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

/// Entry point for the `set_config` instruction.
///
/// Steps:
/// 1. Verify that the caller is the current admin.
/// 2. Perform early, lightweight validation of arguments.
/// 3. Call `Config::apply_update` to mutate the configuration.
/// 4. Emit `ConfigUpdated` event for indexers.
pub fn handle(ctx: Context<SetConfig>, args: SetConfigArgs) -> Result<()> {
    let SetConfig {
        admin,
        mut config,
        system_program: _,
        clock,
    } = ctx.accounts;

    let clock_ref: &Clock = clock;

    // -----------------------------------------------------------------------
    // Admin authority check
    // -----------------------------------------------------------------------

    config.assert_admin(admin)?;

    // -----------------------------------------------------------------------
    // Early validation on provided arguments
    // -----------------------------------------------------------------------

    if let Some(fee_bps) = args.fee_bps {
        if fee_bps > MAX_FEE_BPS {
            return err!(Unit09Error::InvalidFeeBps);
        }
    }

    if let Some(max_modules) = args.max_modules_per_repo {
        if max_modules == 0 {
            return err!(Unit09Error::ValueOutOfRange);
        }
    }

    // -----------------------------------------------------------------------
    // Apply updates to Config
    // -----------------------------------------------------------------------

    config.apply_update(
        args.fee_bps,
        args.max_modules_per_repo,
        args.is_active,
        args.policy_ref,
        clock_ref,
    )?;

    // -----------------------------------------------------------------------
    // Emit ConfigUpdated event
    // -----------------------------------------------------------------------

    emit!(ConfigUpdated {
        admin: config.admin,
        fee_bps: config.fee_bps,
        max_modules_per_repo: config.max_modules_per_repo,
    });

    Ok(())
}
