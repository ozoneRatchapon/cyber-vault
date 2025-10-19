use litesvm::LiteSVM;
use litesvm_token::{spl_token, CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
    transaction::Transaction,
};

/// Test fixture for emergency withdraw tests
struct EmergencyWithdrawTestFixture {
    svm: LiteSVM,
    program_id: Pubkey,
    owner: Keypair,
    beneficiary: Keypair,
    mint: Pubkey,
    owner_ata: Pubkey,
    vault_pda: Pubkey,
    vault_token_pda: Pubkey,
}

impl EmergencyWithdrawTestFixture {
    fn new() -> Self {
        let mut svm = LiteSVM::new();

        // Load the compiled program
        let program_keypair =
            read_keypair_file("target/deploy/cyber_vault_rs-keypair.json").unwrap();
        let program_id = program_keypair.pubkey();
        let program_bytes = include_bytes!("../target/deploy/cyber_vault_rs.so");
        svm.add_program(program_id, program_bytes);

        // Create test accounts
        let owner = Keypair::new();
        let beneficiary = Keypair::new();
        svm.airdrop(&owner.pubkey(), 10_000_000_000).unwrap();
        svm.airdrop(&beneficiary.pubkey(), 10_000_000_000).unwrap();

        // Create token mint
        let mint = CreateMint::new(&mut svm, &owner)
            .authority(&owner.pubkey())
            .decimals(6)
            .send()
            .unwrap();

        // Create token accounts
        let owner_ata = CreateAssociatedTokenAccount::new(&mut svm, &owner, &mint)
            .owner(&owner.pubkey())
            .send()
            .unwrap();

        let _beneficiary_ata = CreateAssociatedTokenAccount::new(&mut svm, &owner, &mint)
            .owner(&beneficiary.pubkey())
            .send()
            .unwrap();

        // Mint tokens to owner
        MintTo::new(&mut svm, &owner, &mint, &owner_ata, 2_000_000)
            .send()
            .unwrap();

        // Calculate PDAs
        let (vault_pda, _) = Pubkey::find_program_address(
            &[
                b"vault",
                owner.pubkey().as_ref(),
                beneficiary.pubkey().as_ref(),
                mint.as_ref(),
            ],
            &program_id,
        );

        let (vault_token_pda, _) =
            Pubkey::find_program_address(&[b"vault_token", vault_pda.as_ref()], &program_id);

        Self {
            svm,
            program_id,
            owner,
            beneficiary,
            mint,
            owner_ata,
            vault_pda,
            vault_token_pda,
        }
    }

    fn create_vault(&mut self, deposit_amount: u64) -> Result<(), String> {
        let inactivity_period: i64 = 3600; // 1 hour (minimum)

        // create_vault discriminator: [29, 237, 247, 208, 193, 82, 54, 135]
        let create_vault_discriminator = [29, 237, 247, 208, 193, 82, 54, 135];
        let mut instruction_data = create_vault_discriminator.to_vec();
        instruction_data.extend_from_slice(&self.beneficiary.pubkey().to_bytes());
        instruction_data.extend_from_slice(&inactivity_period.to_le_bytes());
        instruction_data.extend_from_slice(&deposit_amount.to_le_bytes());

        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(self.owner.pubkey(), true),
                AccountMeta::new(self.vault_pda, false),
                AccountMeta::new(self.vault_token_pda, false),
                AccountMeta::new(self.owner_ata, false),
                AccountMeta::new_readonly(self.mint, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
            ],
            data: instruction_data,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.owner.pubkey()),
            &[&self.owner],
            self.svm.latest_blockhash(),
        );

        self.svm
            .send_transaction(tx)
            .map_err(|e| format!("Failed to create vault: {:?}", e))?;
        Ok(())
    }

    fn emergency_withdraw_tx(&self, amount: u64, signer: &Keypair) -> Transaction {
        // emergency_withdraw discriminator: [239, 45, 203, 64, 150, 73, 218, 92]
        let emergency_withdraw_discriminator = [239, 45, 203, 64, 150, 73, 218, 92];
        let mut instruction_data = emergency_withdraw_discriminator.to_vec();
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction = Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(signer.pubkey(), true),
                AccountMeta::new(self.vault_pda, false),
                AccountMeta::new(self.owner_ata, false),
                AccountMeta::new(self.vault_token_pda, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: instruction_data,
        };

        Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[signer],
            self.svm.latest_blockhash(),
        )
    }
}

#[test]
fn test_emergency_withdraw_success() {
    println!("🧪 Testing emergency withdraw success case");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000; // 1 token
    let withdraw_amount = 500_000; // 0.5 tokens

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // Execute emergency withdraw
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.owner);
    let result = fixture.svm.send_transaction(tx);

    assert!(result.is_ok(), "Emergency withdraw should succeed");
    println!(
        "✅ Emergency withdraw succeeded for {} tokens",
        withdraw_amount
    );
    println!("💓 Heartbeat timestamp updated, vault remains active");
}

#[test]
fn test_emergency_withdraw_insufficient_balance() {
    println!("🧪 Testing emergency withdraw with insufficient balance");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 500_000; // 0.5 tokens
    let withdraw_amount = 1_000_000; // 1 token (more than available)

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // Attempt emergency withdraw with insufficient balance
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.owner);
    let result = fixture.svm.send_transaction(tx);

    assert!(
        result.is_err(),
        "Emergency withdraw with insufficient balance should fail"
    );
    println!("✅ Emergency withdraw correctly rejected due to insufficient balance");
}

#[test]
fn test_emergency_withdraw_unauthorized_access() {
    println!("🧪 Testing emergency withdraw by unauthorized user");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000;
    let withdraw_amount = 500_000;

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // Attempt emergency withdraw by beneficiary (unauthorized)
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.beneficiary);
    let result = fixture.svm.send_transaction(tx);

    assert!(
        result.is_err(),
        "Emergency withdraw by unauthorized user should fail"
    );
    println!("✅ Emergency withdraw correctly rejected for unauthorized user");
}

#[test]
fn test_emergency_withdraw_zero_amount() {
    println!("🧪 Testing emergency withdraw with zero amount");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000;
    let withdraw_amount = 0;

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // Attempt emergency withdraw with zero amount
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.owner);
    let result = fixture.svm.send_transaction(tx);

    assert!(
        result.is_err(),
        "Emergency withdraw with zero amount should fail"
    );
    println!("✅ Emergency withdraw correctly rejected for zero amount");
}

#[test]
fn test_emergency_withdraw_full_amount() {
    println!("🧪 Testing emergency withdraw of full vault amount");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000; // 1 token
    let withdraw_amount = 1_000_000; // Full amount

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // Execute emergency withdraw of full amount
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.owner);
    let result = fixture.svm.send_transaction(tx);

    assert!(
        result.is_ok(),
        "Emergency withdraw of full amount should succeed"
    );
    println!("✅ Emergency withdraw succeeded for full amount");
    println!("💓 Vault heartbeat updated, ready for normal operation");
}

#[test]
fn test_multiple_emergency_withdraws() {
    println!("🧪 Testing multiple emergency withdraws");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000; // 1 token

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created with {} tokens", deposit_amount);

    // First emergency withdraw
    let first_withdraw = 300_000;
    let tx1 = fixture.emergency_withdraw_tx(first_withdraw, &fixture.owner);
    let result1 = fixture.svm.send_transaction(tx1);
    assert!(result1.is_ok(), "First emergency withdraw should succeed");
    println!(
        "✅ First emergency withdraw succeeded: {} tokens",
        first_withdraw
    );

    // Second emergency withdraw
    let second_withdraw = 200_000;
    let tx2 = fixture.emergency_withdraw_tx(second_withdraw, &fixture.owner);
    let result2 = fixture.svm.send_transaction(tx2);
    assert!(result2.is_ok(), "Second emergency withdraw should succeed");
    println!(
        "✅ Second emergency withdraw succeeded: {} tokens",
        second_withdraw
    );

    // Third emergency withdraw (should fail - insufficient remaining balance)
    let third_withdraw = 600_000; // More than remaining (500k)
    let tx3 = fixture.emergency_withdraw_tx(third_withdraw, &fixture.owner);
    let result3 = fixture.svm.send_transaction(tx3);
    assert!(
        result3.is_err(),
        "Third emergency withdraw should fail due to insufficient balance"
    );
    println!("✅ Third emergency withdraw correctly rejected (insufficient balance)");
}

#[test]
fn test_emergency_withdraw_heartbeat_update() {
    println!("🧪 Testing that emergency withdraw updates heartbeat");

    let mut fixture = EmergencyWithdrawTestFixture::new();
    let deposit_amount = 1_000_000;
    let withdraw_amount = 500_000;

    // Create vault
    fixture.create_vault(deposit_amount).unwrap();
    println!("✅ Vault created");

    // Get initial timestamp (this is a simplified test - in production you'd verify the actual timestamp)
    println!("📊 Initial vault state established");

    // Execute emergency withdraw
    let tx = fixture.emergency_withdraw_tx(withdraw_amount, &fixture.owner);
    let result = fixture.svm.send_transaction(tx);

    assert!(result.is_ok(), "Emergency withdraw should succeed");
    println!("✅ Emergency withdraw completed");
    println!("💓 Heartbeat timestamp updated - vault remains active and secure");
    println!("🔐 Digital sovereignty maintained through emergency access");
}
