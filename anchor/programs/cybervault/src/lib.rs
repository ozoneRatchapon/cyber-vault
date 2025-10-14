use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

// Custom error codes for the cyberpunk dead man's switch
#[error_code]
pub enum CyberVaultError {
    #[msg("The vault is still active. Keep-alive signal detected.")]
    VaultStillActive,
    #[msg("Unauthorized. Only the owner can perform this action.")]
    UnauthorizedOwner,
    #[msg("Unauthorized. Only the beneficiary can claim these assets.")]
    UnauthorizedBeneficiary,
    #[msg("Timeout period too short. Minimum 1 hour required.")]
    TimeoutTooShort,
    #[msg("The digital silence has not been long enough. Vault is still protected.")]
    TimeoutNotReached,
    #[msg("Insufficient balance in the vault.")]
    InsufficientBalance,
    #[msg("Cannot set yourself as the beneficiary. Choose another guardian.")]
    SelfBeneficiary,
    #[msg("Vault is no longer active. Assets have been claimed.")]
    VaultInactive,
}

// Constants for the cyberpunk aesthetic and security
pub const MINIMUM_TIMEOUT: i64 = 3600; // 1 hour minimum timeout
pub const VAULT_SEED: &[u8] = b"cyber_vault";
pub const TOKEN_VAULT_SEED: &[u8] = b"token_vault";

#[program]
pub mod cybervault {
    use super::*;

    /// Initialize a new Cyber-Vault with dead man's switch capabilities
    /// The future is now - protect your digital legacy through immutable code
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        beneficiary: Pubkey,
        timeout_seconds: i64,
    ) -> Result<()> {
        // Validate inputs with cyberpunk-style error messages
        require!(
            timeout_seconds >= MINIMUM_TIMEOUT,
            CyberVaultError::TimeoutTooShort
        );
        require!(
            beneficiary != ctx.accounts.owner.key(),
            CyberVaultError::SelfBeneficiary
        );

        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        // Initialize the vault with cyberpunk data
        vault.owner = ctx.accounts.owner.key();
        vault.beneficiary = beneficiary;
        vault.mint = ctx.accounts.mint.key();
        vault.token_vault = ctx.accounts.token_vault.key();
        vault.timeout_seconds = timeout_seconds;
        vault.last_heartbeat = clock.unix_timestamp;
        vault.total_deposited = 0;
        vault.is_active = true;
        vault.bump = ctx.bumps.vault;

        msg!("🔒 Cyber-Vault initialized. Digital assets now protected by immutable code.");
        msg!("Owner: {}", vault.owner);
        msg!("Beneficiary: {}", vault.beneficiary);
        msg!("Timeout: {} seconds", vault.timeout_seconds);

        Ok(())
    }

    /// Deposit SPL tokens into the Cyber-Vault
    /// Your digital fortune is now secured in the blockchain
    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount: u64) -> Result<()> {
        require!(amount > 0, CyberVaultError::InsufficientBalance);
        require!(ctx.accounts.vault.is_active, CyberVaultError::VaultInactive);

        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        // Update heartbeat on deposit (owner activity detected)
        vault.last_heartbeat = clock.unix_timestamp;

        // Transfer tokens from owner to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner_token_account.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        // Update vault state
        vault.total_deposited = vault.total_deposited.checked_add(amount).unwrap();

        msg!(
            "💎 {} tokens secured in Cyber-Vault. Total: {}",
            amount,
            vault.total_deposited
        );
        msg!("⚡ Heartbeat updated. Digital presence confirmed.");

        Ok(())
    }

    /// Send a keep-alive signal to reset the dead man's switch
    /// Proof of life in the digital realm
    pub fn send_heartbeat(ctx: Context<SendHeartbeat>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let clock = Clock::get()?;

        require!(vault.is_active, CyberVaultError::VaultInactive);

        // Reset the timer - owner is still alive in cyberspace
        vault.last_heartbeat = clock.unix_timestamp;

        msg!("💓 Heartbeat detected. Digital presence confirmed.");
        msg!("⏰ Dead man's switch reset. Vault remains secured.");

        Ok(())
    }

    /// Claim the digital inheritance after the timeout period
    /// The silence of cyberspace has spoken
    pub fn claim_inheritance(ctx: Context<ClaimInheritance>) -> Result<()> {
        let clock = Clock::get()?;

        // Get immutable references first
        let is_active = ctx.accounts.vault.is_active;
        let last_heartbeat = ctx.accounts.vault.last_heartbeat;
        let timeout_seconds = ctx.accounts.vault.timeout_seconds;
        let total_amount = ctx.accounts.vault.total_deposited;
        let owner_key = ctx.accounts.vault.owner.key();
        let vault_bump = ctx.accounts.vault.bump;
        let beneficiary = ctx.accounts.vault.beneficiary;

        require!(is_active, CyberVaultError::VaultInactive);

        // Check if enough time has passed in the digital silence
        let time_elapsed = clock.unix_timestamp - last_heartbeat;
        require!(
            time_elapsed >= timeout_seconds,
            CyberVaultError::TimeoutNotReached
        );

        // The digital will is now executed
        require!(total_amount > 0, CyberVaultError::InsufficientBalance);

        // Create PDA signer for the vault
        let seeds = &[VAULT_SEED, owner_key.as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];

        // Get authority account info before mutable borrow
        let vault_authority = ctx.accounts.vault.to_account_info();

        // Transfer all tokens to the beneficiary
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.beneficiary_token_account.to_account_info(),
            authority: vault_authority,
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, total_amount)?;

        // Mark vault as inactive - the digital will has been executed
        let vault = &mut ctx.accounts.vault;
        vault.is_active = false;
        vault.total_deposited = 0;

        msg!("💀 Digital silence detected. Dead man's switch activated.");
        msg!(
            "🎭 {} tokens transferred to beneficiary: {}",
            total_amount,
            beneficiary
        );
        msg!("⚰️ Cyber-Vault legacy protocol complete. Code is Law.");

        Ok(())
    }

    /// Emergency withdrawal by owner (if still alive)
    /// Take back your digital sovereignty
    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>, amount: u64) -> Result<()> {
        let clock = Clock::get()?;

        // Get immutable references first
        let is_active = ctx.accounts.vault.is_active;
        let total_deposited = ctx.accounts.vault.total_deposited;
        let owner_key = ctx.accounts.vault.owner.key();
        let vault_bump = ctx.accounts.vault.bump;

        require!(is_active, CyberVaultError::VaultInactive);
        require!(
            amount <= total_deposited,
            CyberVaultError::InsufficientBalance
        );

        // Create PDA signer for the vault
        let seeds = &[VAULT_SEED, owner_key.as_ref(), &[vault_bump]];
        let signer = &[&seeds[..]];

        // Get authority account info before mutable borrow
        let vault_authority = ctx.accounts.vault.to_account_info();

        // Transfer tokens back to owner
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: vault_authority,
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount)?;

        // Update vault state with mutable reference
        let vault = &mut ctx.accounts.vault;
        vault.last_heartbeat = clock.unix_timestamp;
        vault.total_deposited = total_deposited.checked_sub(amount).unwrap();

        msg!(
            "🚨 Emergency withdrawal executed. {} tokens reclaimed by owner.",
            amount
        );
        msg!("💓 Heartbeat updated. Digital sovereignty maintained.");

        Ok(())
    }
}

// The Cyber-Vault state - immutable record of digital inheritance
#[account]
pub struct CyberVault {
    pub owner: Pubkey,        // The digital entity who owns the vault
    pub beneficiary: Pubkey,  // The chosen inheritor of digital assets
    pub mint: Pubkey,         // The SPL token mint address
    pub token_vault: Pubkey,  // The token account holding the assets
    pub timeout_seconds: i64, // Silence threshold in seconds
    pub last_heartbeat: i64,  // Unix timestamp of last activity
    pub total_deposited: u64, // Total tokens secured in the vault
    pub is_active: bool,      // Whether the vault is operational
    pub bump: u8,             // PDA bump seed
}

impl CyberVault {
    pub const LEN: usize = 8 + // Discriminator
        32 + // owner
        32 + // beneficiary
        32 + // mint
        32 + // token_vault
        8 +  // timeout_seconds
        8 +  // last_heartbeat
        8 +  // total_deposited
        1 +  // is_active
        1; // bump
}

// Account validation contexts with cyberpunk naming

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        space = CyberVault::LEN,
        seeds = [VAULT_SEED, owner.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, CyberVault>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = vault,
        seeds = [TOKEN_VAULT_SEED, owner.key().as_ref()],
        bump
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner @ CyberVaultError::UnauthorizedOwner,
        has_one = mint,
        has_one = token_vault,
    )]
    pub vault: Account<'info, CyberVault>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SendHeartbeat<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner @ CyberVaultError::UnauthorizedOwner,
    )]
    pub vault: Account<'info, CyberVault>,
}

#[derive(Accounts)]
pub struct ClaimInheritance<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mut,
        has_one = beneficiary @ CyberVaultError::UnauthorizedBeneficiary,
        has_one = mint,
        has_one = token_vault,
    )]
    pub vault: Account<'info, CyberVault>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = beneficiary,
    )]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner @ CyberVaultError::UnauthorizedOwner,
        has_one = mint,
        has_one = token_vault,
    )]
    pub vault: Account<'info, CyberVault>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = vault,
    )]
    pub token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
