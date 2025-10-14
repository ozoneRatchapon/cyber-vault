#!/usr/bin/env node
/**
 * 🔐 CYBER-VAULT DEMO SCRIPT
 *
 * Decentralized Dead Man's Switch Protocol Demonstration
 *
 * This script demonstrates the key functionalities of the Cyber-Vault
 * smart contract including vault creation, token deposits, heartbeat
 * signals, and inheritance claims.
 *
 * "Code is Law - Your Digital Legacy Protected by Mathematics"
 */

const { Connection, Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } = require('@solana/web3.js')
const { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } = require('@solana/spl-token')

// Program configuration
const PROGRAM_ID = new PublicKey('JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H')
const CYBER_VAULT_SEED = 'cyber_vault'
const TOKEN_VAULT_SEED = 'token_vault'
const MINIMUM_TIMEOUT = 3600 // 1 hour in seconds

class CyberVaultDemo {
  constructor(rpcUrl = 'http://localhost:8899') {
    this.connection = new Connection(rpcUrl, 'confirmed')
    this.owner = null
    this.beneficiary = null
    this.mint = null
    this.vaultPDA = null
    this.tokenVaultPDA = null
  }

  /**
   * Initialize the cyberpunk demo environment
   */
  async initialize() {
    console.log('🌐 Initializing Cyber-Vault Demo Environment...\n')

    // Generate keypairs for demo
    this.owner = Keypair.generate()
    this.beneficiary = Keypair.generate()

    console.log('👤 Demo Participants:')
    console.log(`   Owner: ${this.owner.publicKey.toString()}`)
    console.log(`   Beneficiary: ${this.beneficiary.publicKey.toString()}\n`)

    // Airdrop SOL for transactions
    await this.airdropSol(this.owner.publicKey, 2)
    await this.airdropSol(this.beneficiary.publicKey, 1)

    // Create test token mint
    await this.createTestMint()

    // Derive PDA addresses
    this.derivePDAs()

    console.log('✅ Environment initialized successfully!\n')
  }

  /**
   * Airdrop SOL to an account
   */
  async airdropSol(publicKey, solAmount) {
    console.log(`💰 Airdropping ${solAmount} SOL to ${publicKey.toString().slice(0, 8)}...`)
    const signature = await this.connection.requestAirdrop(publicKey, solAmount * LAMPORTS_PER_SOL)
    await this.connection.confirmTransaction(signature)
    console.log(`   ✅ Airdrop confirmed: ${signature}\n`)
  }

  /**
   * Create a test token mint for demonstration
   */
  async createTestMint() {
    console.log('🪙 Creating test token mint (DEMO tokens)...')
    this.mint = await createMint(
      this.connection,
      this.owner,
      this.owner.publicKey,
      null,
      6, // decimals
    )
    console.log(`   ✅ Mint created: ${this.mint.toString()}\n`)

    // Create token accounts
    this.ownerTokenAccount = await createAccount(this.connection, this.owner, this.mint, this.owner.publicKey)

    this.beneficiaryTokenAccount = await createAccount(
      this.connection,
      this.beneficiary,
      this.mint,
      this.beneficiary.publicKey,
    )

    // Mint tokens to owner
    await mintTo(
      this.connection,
      this.owner,
      this.mint,
      this.ownerTokenAccount,
      this.owner,
      10_000_000, // 10 DEMO tokens
    )

    console.log('💎 Token accounts created and funded')
    console.log(`   Owner token account: ${this.ownerTokenAccount.toString()}`)
    console.log(`   Beneficiary token account: ${this.beneficiaryTokenAccount.toString()}\n`)
  }

  /**
   * Derive Program Derived Addresses (PDAs)
   */
  derivePDAs() {
    console.log('🔑 Deriving Program Derived Addresses...')

    ;[this.vaultPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(CYBER_VAULT_SEED), this.owner.publicKey.toBuffer()],
      PROGRAM_ID,
    )

    ;[this.tokenVaultPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from(TOKEN_VAULT_SEED), this.owner.publicKey.toBuffer()],
      PROGRAM_ID,
    )

    console.log(`   Vault PDA: ${this.vaultPDA.toString()}`)
    console.log(`   Token Vault PDA: ${this.tokenVaultPDA.toString()}\n`)
  }

  /**
   * Demonstrate vault initialization
   */
  async demoInitializeVault() {
    console.log('🔒 DEMO: Initializing Cyber-Vault...')
    console.log("   This creates a new dead man's switch vault")
    console.log('   Setting 1 hour timeout for demonstration\n')

    // In a real implementation, you would construct and send the transaction
    console.log('📝 Transaction would include:')
    console.log('   - Instruction: initialize_vault')
    console.log(`   - Beneficiary: ${this.beneficiary.publicKey.toString()}`)
    console.log('   - Timeout: 3600 seconds (1 hour)')
    console.log('   - Accounts: owner, vault, token_vault, mint, system_program\n')

    // Simulate transaction
    await this.simulateTransaction('initialize_vault')

    console.log('✅ Vault initialized successfully!')
    console.log('   Your digital legacy is now protected by immutable code\n')
  }

  /**
   * Demonstrate token deposit
   */
  async demoDepositTokens() {
    console.log('💎 DEMO: Depositing tokens into Cyber-Vault...')
    console.log('   Securing 5 DEMO tokens in the blockchain vault')
    console.log('   This also updates the heartbeat timestamp\n')

    const depositAmount = 5_000_000 // 5 tokens with 6 decimals

    console.log('📝 Transaction would include:')
    console.log('   - Instruction: deposit_tokens')
    console.log(`   - Amount: ${depositAmount / 1_000_000} DEMO tokens`)
    console.log('   - Transfer from owner to vault token account')
    console.log('   - Heartbeat update (proof of life)\n')

    await this.simulateTransaction('deposit_tokens')

    console.log('✅ Tokens deposited successfully!')
    console.log('   Digital assets secured in cyber-vault')
    console.log('   💓 Heartbeat updated - you are alive in cyberspace\n')
  }

  /**
   * Demonstrate heartbeat signal
   */
  async demoSendHeartbeat() {
    console.log('💓 DEMO: Sending heartbeat signal...')
    console.log('   Proof of life in the digital realm')
    console.log("   Resets the dead man's switch timer\n")

    console.log('📝 Transaction would include:')
    console.log('   - Instruction: send_heartbeat')
    console.log('   - Updates last_heartbeat timestamp')
    console.log('   - Resets countdown timer\n')

    await this.simulateTransaction('send_heartbeat')

    console.log('✅ Heartbeat confirmed!')
    console.log('   ⚡ Digital presence validated')
    console.log("   🔄 Dead man's switch timer reset\n")
  }

  /**
   * Demonstrate inheritance claim (after timeout)
   */
  async demoClaimInheritance() {
    console.log('💀 DEMO: Claiming digital inheritance...')
    console.log('   Simulating scenario where timeout has expired')
    console.log('   The silence of cyberspace has been detected\n')

    console.log('📝 Transaction would include:')
    console.log('   - Instruction: claim_inheritance')
    console.log('   - Verify timeout period has passed')
    console.log('   - Transfer all tokens to beneficiary')
    console.log('   - Mark vault as inactive\n')

    await this.simulateTransaction('claim_inheritance')

    console.log('⚰️  Digital inheritance executed!')
    console.log('   🎭 Assets transferred to beneficiary')
    console.log('   ⚖️  Code is Law - cyberpunk legacy complete\n')
  }

  /**
   * Demonstrate emergency withdrawal
   */
  async demoEmergencyWithdraw() {
    console.log('🚨 DEMO: Emergency withdrawal by owner...')
    console.log('   Owner reclaiming digital sovereignty')
    console.log('   Withdrawing 2 DEMO tokens while still alive\n')

    const withdrawAmount = 2_000_000 // 2 tokens

    console.log('📝 Transaction would include:')
    console.log('   - Instruction: emergency_withdraw')
    console.log(`   - Amount: ${withdrawAmount / 1_000_000} DEMO tokens`)
    console.log('   - Transfer from vault back to owner')
    console.log('   - Update heartbeat timestamp\n')

    await this.simulateTransaction('emergency_withdraw')

    console.log('✅ Emergency withdrawal successful!')
    console.log('   💰 Tokens reclaimed by owner')
    console.log('   💓 Heartbeat updated - still alive!\n')
  }

  /**
   * Display vault status
   */
  async displayVaultStatus() {
    console.log('📊 CYBER-VAULT STATUS DASHBOARD')
    console.log('═══════════════════════════════════════')

    // Mock vault data for demonstration
    const mockStatus = {
      owner: this.owner.publicKey.toString(),
      beneficiary: this.beneficiary.publicKey.toString(),
      isActive: true,
      totalDeposited: 5.0,
      timeoutSeconds: 3600,
      lastHeartbeat: new Date(),
      timeUntilTimeout: 3240, // 54 minutes
    }

    console.log(`🔒 Vault Address: ${this.vaultPDA.toString()}`)
    console.log(`👤 Owner: ${mockStatus.owner.slice(0, 8)}...${mockStatus.owner.slice(-8)}`)
    console.log(`🎭 Beneficiary: ${mockStatus.beneficiary.slice(0, 8)}...${mockStatus.beneficiary.slice(-8)}`)
    console.log(`💎 Total Deposited: ${mockStatus.totalDeposited} DEMO`)
    console.log(`⏰ Timeout Period: ${mockStatus.timeoutSeconds / 3600} hours`)
    console.log(`💓 Last Heartbeat: ${mockStatus.lastHeartbeat.toLocaleString()}`)
    console.log(`⌛ Time Until Timeout: ${Math.floor(mockStatus.timeUntilTimeout / 60)} minutes`)
    console.log(`🔋 Status: ${mockStatus.isActive ? '🟢 ACTIVE' : '🔴 INACTIVE'}`)
    console.log('\n')
  }

  /**
   * Simulate a transaction with delay for demo purposes
   */
  async simulateTransaction(instruction) {
    process.stdout.write('   📡 Broadcasting transaction')
    for (let i = 0; i < 3; i++) {
      process.stdout.write('.')
      await this.sleep(500)
    }
    console.log(' ✅')

    // Simulate confirmation time
    process.stdout.write('   ⏳ Awaiting confirmation')
    for (let i = 0; i < 5; i++) {
      process.stdout.write('.')
      await this.sleep(300)
    }
    console.log(' ✅\n')
  }

  /**
   * Sleep utility
   */
  sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms))
  }

  /**
   * Display cyberpunk banner
   */
  displayBanner() {
    console.log('\n')
    console.log('╔═══════════════════════════════════════════════════════════════╗')
    console.log('║                     🔐 CYBER-VAULT                           ║')
    console.log("║               Decentralized Dead Man's Switch                ║")
    console.log('║                                                               ║')
    console.log('║     "Your Digital Legacy Protected by Immutable Code"        ║')
    console.log('║                                                               ║')
    console.log('║  💀 Code is Law  •  🔒 Trustless Security  •  ⚡ Instant    ║')
    console.log('╚═══════════════════════════════════════════════════════════════╝')
    console.log('\n')
  }

  /**
   * Display security warnings
   */
  displaySecurityWarnings() {
    console.log('⚠️  SECURITY WARNINGS:')
    console.log('   • This is a Proof of Concept for demonstration only')
    console.log('   • Do not use with real funds on mainnet')
    console.log('   • Always audit smart contracts before production use')
    console.log('   • Test thoroughly on devnet before mainnet deployment\n')
  }

  /**
   * Run the complete demo
   */
  async runDemo() {
    try {
      this.displayBanner()
      this.displaySecurityWarnings()

      // Initialize demo environment
      await this.initialize()

      // Run through all demo scenarios
      console.log('🎮 Starting Cyber-Vault Protocol Demonstration...\n')

      await this.demoInitializeVault()
      await this.displayVaultStatus()

      await this.demoDepositTokens()
      await this.displayVaultStatus()

      await this.demoSendHeartbeat()

      await this.demoEmergencyWithdraw()

      // Simulate inheritance scenario
      console.log('⏳ Simulating digital death scenario...')
      console.log('   (In reality, you would wait for the timeout period)\n')
      await this.sleep(2000)

      await this.demoClaimInheritance()

      console.log('🎉 DEMO COMPLETE!')
      console.log('═════════════════════════════════════════════')
      console.log('You have successfully witnessed the future of digital inheritance!')
      console.log('')
      console.log('Key Features Demonstrated:')
      console.log('✅ Trustless vault creation')
      console.log('✅ Secure token deposits')
      console.log('✅ Heartbeat keep-alive mechanism')
      console.log('✅ Emergency withdrawal controls')
      console.log('✅ Automatic inheritance execution')
      console.log('')
      console.log('🔮 Welcome to the cyberpunk age of decentralized legacy protection!')
      console.log('💀 "In crypto we trust, in code we verify, in math we believe"')
      console.log('\n')
    } catch (error) {
      console.error('❌ Demo Error:', error.message)
      console.error('Please ensure you have a local Solana validator running')
      console.error('Run: solana-test-validator')
    }
  }
}

// Execute demo if run directly
if (require.main === module) {
  const demo = new CyberVaultDemo()
  demo
    .runDemo()
    .then(() => {
      process.exit(0)
    })
    .catch((error) => {
      console.error('Fatal Error:', error)
      process.exit(1)
    })
}

module.exports = CyberVaultDemo
