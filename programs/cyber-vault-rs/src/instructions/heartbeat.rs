use crate::state::Vault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Heartbeat<'info> {
    #[account(
        mut,
        seeds = [b"vault", vault.owner.as_ref(), vault.beneficiary.as_ref(), vault.token_mint.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,

    pub owner: Signer<'info>,
}

impl<'info> Heartbeat<'info> {
    pub fn send_heartbeat(&mut self) -> Result<()> {
        let clock = Clock::get()?;

        require!(
            self.owner.key() == self.vault.owner,
            crate::error::VaultError::UnauthorizedAccess
        );

        self.vault.last_heartbeat = clock.unix_timestamp;

        msg!("💓 Heartbeat detected. Digital presence confirmed.");
        msg!("⏰ Dead man's switch reset. Vault remains secured.");

        Ok(())
    }
}
