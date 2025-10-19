use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub beneficiary: Pubkey,
    pub token_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub inactivity_period: i64,
    pub last_heartbeat: i64,
    pub is_active: bool,
    pub bump: u8,
}

impl Vault {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 1 + 1;
}
