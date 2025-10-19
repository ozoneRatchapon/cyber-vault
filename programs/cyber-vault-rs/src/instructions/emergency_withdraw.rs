use crate::state::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            crate::VAULT_SEED,
            vault.owner.as_ref(),
            vault.beneficiary.as_ref(),
            vault.token_mint.as_ref(),
        ],
        bump = vault.bump,
        has_one = owner @ crate::error::VaultError::UnauthorizedAccess,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        token::mint = vault.token_mint,
        token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = vault.token_mint,
        token::authority = vault,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> EmergencyWithdraw<'info> {
    pub fn emergency_withdraw(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, crate::error::VaultError::InsufficientBalance);
        require!(
            self.vault.is_active,
            crate::error::VaultError::VaultInactive
        );
        require!(
            amount <= self.vault_token_account.amount,
            crate::error::VaultError::InsufficientBalance
        );

        let clock = Clock::get()?;

        // Get vault authority before mutable borrow
        let vault_authority = self.vault.to_account_info();

        // Create PDA signer for the vault
        let seeds = &[
            crate::VAULT_SEED,
            self.vault.owner.as_ref(),
            self.vault.beneficiary.as_ref(),
            self.vault.token_mint.as_ref(),
            &[self.vault.bump],
        ];
        let signer = &[&seeds[..]];

        // Transfer tokens back to owner
        let cpi_accounts = Transfer {
            from: self.vault_token_account.to_account_info(),
            to: self.owner_token_account.to_account_info(),
            authority: vault_authority,
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount)?;

        // Update vault state
        self.vault.last_heartbeat = clock.unix_timestamp;

        msg!(
            "🚨 Emergency withdrawal executed. {} tokens reclaimed by owner.",
            amount
        );
        msg!("💓 Heartbeat updated. Digital sovereignty maintained.");

        Ok(())
    }
}
