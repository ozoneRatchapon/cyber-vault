use crate::wallet::WalletProvider;

use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

// Program ID from IDL
const PROGRAM_ID: &str = "5QTdo3dK7pQZuYrL9ZCUWzAywpohu3gGEJBmbxqAA1gW";

// Token Program ID
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

// System Program ID
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

// Rent Sysvar ID
const RENT_ID: &str = "SysvarRent111111111111111111111111111111111";

pub struct VaultOperations {
    pub program_id: Pubkey,
    pub wallet: WalletProvider,
}

impl VaultOperations {
    pub fn new(wallet: WalletProvider) -> Result<Self, String> {
        let program_id =
            Pubkey::from_str(PROGRAM_ID).map_err(|e| format!("Invalid program ID: {}", e))?;

        Ok(Self { program_id, wallet })
    }

    // Create vault instruction discriminator
    fn create_vault_discriminator() -> Vec<u8> {
        vec![29, 237, 247, 208, 193, 82, 54, 135]
    }

    // Heartbeat instruction discriminator
    fn heartbeat_discriminator() -> Vec<u8> {
        vec![202, 104, 56, 6, 240, 170, 63, 134]
    }

    // Claim instruction discriminator
    fn claim_discriminator() -> Vec<u8> {
        vec![62, 198, 214, 193, 213, 159, 108, 210]
    }

    // Emergency withdraw instruction discriminator
    fn emergency_withdraw_discriminator() -> Vec<u8> {
        vec![239, 45, 203, 64, 150, 73, 218, 92]
    }

    // Get vault PDA seeds
    fn get_vault_seeds(owner: &Pubkey, beneficiary: &Pubkey, token_mint: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"vault".to_vec(),
            owner.to_bytes().to_vec(),
            beneficiary.to_bytes().to_vec(),
            token_mint.to_bytes().to_vec(),
        ]
    }

    // Get vault token account PDA seeds
    fn get_vault_token_seeds(vault: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"vault_token".to_vec(), vault.to_bytes().to_vec()]
    }

    // Find vault PDA
    pub fn find_vault_pda(
        owner: &Pubkey,
        beneficiary: &Pubkey,
        token_mint: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds = Self::get_vault_seeds(owner, beneficiary, token_mint);
        Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            &Pubkey::from_str(PROGRAM_ID).unwrap(),
        )
    }

    // Find vault token account PDA
    pub fn find_vault_token_pda(vault: &Pubkey) -> (Pubkey, u8) {
        let seeds = Self::get_vault_token_seeds(vault);
        Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            &Pubkey::from_str(PROGRAM_ID).unwrap(),
        )
    }

    // Create vault instruction
    pub fn create_vault_instruction(
        &self,
        owner: &Pubkey,
        beneficiary: &Pubkey,
        token_mint: &Pubkey,
        inactivity_period: i64,
        amount: u64,
    ) -> Result<Instruction, String> {
        let (vault, _vault_bump) = Self::find_vault_pda(owner, beneficiary, token_mint);
        let (vault_token_account, _vault_token_bump) = Self::find_vault_token_pda(&vault);

        let mut instruction_data = Self::create_vault_discriminator();
        instruction_data.extend_from_slice(&beneficiary.to_bytes());
        instruction_data.extend_from_slice(&inactivity_period.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(*owner, true),
            solana_sdk::instruction::AccountMeta::new(vault, false),
            solana_sdk::instruction::AccountMeta::new(vault_token_account, false),
            solana_sdk::instruction::AccountMeta::new(*owner, false), // owner_token_account (will be set by frontend)
            solana_sdk::instruction::AccountMeta::new_readonly(*token_mint, false),
            solana_sdk::instruction::AccountMeta::new_readonly(
                Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new_readonly(
                Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
                false,
            ),
            solana_sdk::instruction::AccountMeta::new_readonly(
                Pubkey::from_str(RENT_ID).unwrap(),
                false,
            ),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    // Heartbeat instruction
    pub fn heartbeat_instruction(
        &self,
        owner: &Pubkey,
        beneficiary: &Pubkey,
        token_mint: &Pubkey,
    ) -> Result<Instruction, String> {
        let (vault, _vault_bump) = Self::find_vault_pda(owner, beneficiary, token_mint);

        let instruction_data = Self::heartbeat_discriminator();

        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(vault, false),
            solana_sdk::instruction::AccountMeta::new_readonly(*owner, true),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    // Claim instruction
    pub fn claim_instruction(
        &self,
        owner: &Pubkey,
        beneficiary: &Pubkey,
        token_mint: &Pubkey,
        beneficiary_token_account: &Pubkey,
    ) -> Result<Instruction, String> {
        let (vault, _vault_bump) = Self::find_vault_pda(owner, beneficiary, token_mint);
        let (vault_token_account, _vault_token_bump) = Self::find_vault_token_pda(&vault);

        let instruction_data = Self::claim_discriminator();

        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(vault, false),
            solana_sdk::instruction::AccountMeta::new(vault_token_account, false),
            solana_sdk::instruction::AccountMeta::new(*beneficiary_token_account, false),
            solana_sdk::instruction::AccountMeta::new(*beneficiary, true),
            solana_sdk::instruction::AccountMeta::new_readonly(
                Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(),
                false,
            ),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    // Emergency withdraw instruction
    pub fn emergency_withdraw_instruction(
        &self,
        owner: &Pubkey,
        beneficiary: &Pubkey,
        token_mint: &Pubkey,
        owner_token_account: &Pubkey,
        amount: u64,
    ) -> Result<Instruction, String> {
        let (vault, _vault_bump) = Self::find_vault_pda(owner, beneficiary, token_mint);
        let (vault_token_account, _vault_token_bump) = Self::find_vault_token_pda(&vault);

        let mut instruction_data = Self::emergency_withdraw_discriminator();
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(*owner, true),
            solana_sdk::instruction::AccountMeta::new(vault, false),
            solana_sdk::instruction::AccountMeta::new(*owner_token_account, false),
            solana_sdk::instruction::AccountMeta::new(vault_token_account, false),
            solana_sdk::instruction::AccountMeta::new_readonly(
                Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(),
                false,
            ),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    // Get current timestamp
    pub fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    // Create and sign transaction
    pub async fn create_and_sign_transaction(
        &self,
        instructions: Vec<Instruction>,
        fee_payer: &Pubkey,
    ) -> Result<Vec<u8>, String> {
        // Create transaction
        let mut transaction = Transaction::new_with_payer(&instructions, Some(fee_payer));

        // Set recent blockhash (in a real app, you'd get this from the network)
        let recent_blockhash = Hash::default(); // Placeholder
        transaction.sign(
            &[&Keypair::new()], // This would be the user's keypair in a real implementation
            recent_blockhash,
        );

        // Serialize transaction
        let serialized = bincode::serialize(&transaction)
            .map_err(|e| format!("Failed to serialize transaction: {}", e))?;

        // Sign transaction with wallet
        let signed = self.wallet.sign_transaction(&serialized).await?;

        Ok(signed)
    }

    // Send transaction
    pub async fn send_transaction(&self, _signed_transaction: Vec<u8>) -> Result<String, String> {
        // In a real implementation, you would send this to the Solana network
        // For now, we'll just return a mock signature
        let signature = format!(
            "mock_signature_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        Ok(signature)
    }
}

// Utility functions for the frontend
pub mod utils {
    use super::*;
    use std::str::FromStr;

    pub fn validate_pubkey(input: &str) -> Result<Pubkey, String> {
        Pubkey::from_str(input).map_err(|_| "Invalid public key".to_string())
    }

    pub fn format_amount(amount: u64, decimals: u8) -> String {
        let divisor = 10u64.pow(decimals as u32);
        let whole = amount / divisor;
        let fractional = amount % divisor;
        if fractional == 0 {
            whole.to_string()
        } else {
            format!(
                "{}.{:0width$}",
                whole,
                fractional,
                width = decimals as usize
            )
        }
    }

    pub fn parse_amount(input: &str, decimals: u8) -> Result<u64, String> {
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() > 2 {
            return Err("Invalid amount format".to_string());
        }

        let whole: u64 = parts[0].parse().map_err(|_| "Invalid amount".to_string())?;
        let fractional = if parts.len() == 2 {
            let frac_str = &parts[1];
            let frac_len = frac_str.len().min(decimals as usize);
            let mut padded = format!(
                "{:0<width$}",
                &frac_str[..frac_len],
                width = decimals as usize
            );
            padded.truncate(decimals as usize);
            padded.parse().unwrap_or(0)
        } else {
            0
        };

        Ok(whole * 10u64.pow(decimals as u32) + fractional)
    }

    pub fn get_token_mint_symbol(mint: &Pubkey) -> &'static str {
        match mint.to_string().as_str() {
            "So11111111111111111111111111111111111111112" => "SOL",
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => "USDC",
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => "USDT",
            _ => "Unknown",
        }
    }
}
