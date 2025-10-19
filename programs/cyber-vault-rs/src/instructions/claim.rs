use crate::state::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.owner.as_ref(), vault.beneficiary.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault_token", vault.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub beneficiary: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Claim<'info> {
    pub fn claim_tokens(&mut self) -> Result<()> {
        let clock = Clock::get()?;

        // Get all immutable data first before any mutable borrow
        let vault_key = self.vault.key();
        let vault_beneficiary = self.vault.beneficiary;
        let vault_balance = self.vault_token_account.amount;
        let vault_info = self.vault.to_account_info();
        let vault_lamports = vault_info.lamports();
        let vault_authority = self.vault.to_account_info();

        // Check if inactivity period has expired
        let time_elapsed = clock.unix_timestamp - self.vault.last_heartbeat;
        require!(
            time_elapsed >= self.vault.inactivity_period,
            crate::error::VaultError::VaultNotExpired
        );

        require!(vault_balance > 0, crate::error::VaultError::EmptyVault);

        // Transfer all tokens to beneficiary
        let seeds = &[
            b"vault",
            self.vault.owner.as_ref(),
            self.vault.beneficiary.as_ref(),
            self.vault.token_mint.as_ref(),
            &[self.vault.bump],
        ];
        let signer = &[&seeds[..]];

        require!(vault_balance > 0, crate::error::VaultError::EmptyVault);

        let cpi_accounts = Transfer {
            from: self.vault_token_account.to_account_info(),
            to: self.beneficiary_token_account.to_account_info(),
            authority: vault_authority.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, vault_balance)?;

        // Close the vault token account
        token::close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: self.vault_token_account.to_account_info(),
                destination: self.beneficiary.to_account_info(),
                authority: vault_authority.clone(),
            },
            signer,
        ))?;

        // Close the vault account (rent reclaim)
        **self.vault.to_account_info().try_borrow_mut_lamports()? -= vault_lamports;
        **self.beneficiary.try_borrow_mut_lamports()? += vault_lamports;

        // Mark vault as inactive - the digital will has been executed
        self.vault.is_active = false;

        msg!("💀 Digital silence detected. Dead man's switch activated.");
        msg!("Vault claimed: {}", vault_key);
        msg!("Beneficiary: {}", vault_beneficiary);
        msg!("Amount claimed: {}", vault_balance);
        msg!("⚰️ Cyber-Vault legacy protocol complete. Code is Law.");

        Ok(())
    }
}
