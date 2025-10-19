use crate::state::Vault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(beneficiary: Pubkey, inactivity_period: i64, amount: u64)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Vault::INIT_SPACE,
        seeds = [b"vault", owner.key().as_ref(), beneficiary.as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = owner,
        token::mint = token_mint,
        token::authority = vault,
        seeds = [b"vault_token", vault.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateVault<'info> {
    pub fn create_vault(
        &mut self,
        beneficiary: Pubkey,
        inactivity_period: i64,
        amount: u64,
        bumps: &CreateVaultBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;

        // Initialize vault
        self.vault.owner = self.owner.key();
        self.vault.beneficiary = beneficiary;
        self.vault.token_mint = self.token_mint.key();
        self.vault.vault_token_account = self.vault_token_account.key();
        self.vault.inactivity_period = inactivity_period;
        self.vault.last_heartbeat = clock.unix_timestamp;
        self.vault.is_active = true;
        self.vault.bump = bumps.vault;

        // Transfer tokens from owner to vault
        let cpi_accounts = Transfer {
            from: self.owner_token_account.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            authority: self.owner.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        msg!("🔒 Cyber-Vault initialized. Digital assets now protected by immutable code.");
        msg!("Owner: {}", self.vault.owner);
        msg!("Beneficiary: {}", beneficiary);
        msg!("Timeout: {} seconds", inactivity_period);
        msg!("Amount locked: {}", amount);

        Ok(())
    }
}
