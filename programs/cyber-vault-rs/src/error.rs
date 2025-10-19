use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized access to vault")]
    UnauthorizedAccess,
    #[msg("Vault inactivity period has not expired yet")]
    VaultNotExpired,
    #[msg("Vault is empty - no tokens to claim")]
    EmptyVault,
    #[msg("The vault is still active. Keep-alive signal detected.")]
    VaultStillActive,
    #[msg("Timeout period too short. Minimum 1 hour required.")]
    TimeoutTooShort,
    #[msg("Cannot set yourself as the beneficiary. Choose another guardian.")]
    SelfBeneficiary,
    #[msg("Vault is no longer active. Assets have been claimed.")]
    VaultInactive,
    #[msg("Insufficient balance in the vault.")]
    InsufficientBalance,
}
