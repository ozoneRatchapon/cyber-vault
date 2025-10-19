use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;

use instructions::*;

declare_id!("7y2rwbCLUSnNsorWWsoRsHyRjKZoH5x9G2R3ERhzPYgy");

// Security constants
pub const MINIMUM_TIMEOUT: i64 = 3600; // 1 hour minimum timeout
pub const VAULT_SEED: &[u8] = b"vault";
pub const TOKEN_VAULT_SEED: &[u8] = b"vault_token";

#[program]
pub mod cyber_vault_rs {
    use super::*;

    pub fn create_vault(
        ctx: Context<CreateVault>,
        beneficiary: Pubkey,
        inactivity_period: i64, // in seconds
        amount: u64,
    ) -> Result<()> {
        // Validate inputs
        require!(
            inactivity_period >= MINIMUM_TIMEOUT,
            crate::error::VaultError::TimeoutTooShort
        );
        require!(
            beneficiary != ctx.accounts.owner.key(),
            crate::error::VaultError::SelfBeneficiary
        );

        ctx.accounts
            .create_vault(beneficiary, inactivity_period, amount, &ctx.bumps)
    }

    pub fn heartbeat(ctx: Context<Heartbeat>) -> Result<()> {
        ctx.accounts.send_heartbeat()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim_tokens()
    }

    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>, amount: u64) -> Result<()> {
        ctx.accounts.emergency_withdraw(amount)
    }
}
