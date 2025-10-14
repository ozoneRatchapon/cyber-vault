# 🚀 Cyber-Vault Deployment Guide

> **"Code is Law - Deploy Your Digital Legacy Protection"**

This guide walks you through deploying and using the Cyber-Vault decentralized dead man's switch on Solana.

## 🎯 Quick Start

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Solana CLI 1.18+
- Anchor Framework 0.30+
- Git

### 1. Clone and Setup

```bash
git clone https://github.com/your-org/cyber-vault.git
cd cyber-vault

# Install dependencies
npm install

# Build the Anchor program
npm run anchor-build

# Generate client code
npm run codama:js
```

### 2. Configure Solana CLI

```bash
# Set to devnet for testing
solana config set --url devnet

# Create a new keypair (or use existing)
solana-keygen new --outfile ~/.config/solana/id.json

# Get some SOL for testing
solana airdrop 2
```

### 3. Deploy the Program

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Note the program ID and update if needed
anchor keys sync
```

### 4. Run the Frontend

```bash
npm run dev
```

Visit `http://localhost:5173` to interact with your deployed Cyber-Vault!

## 🏗️ Detailed Deployment Steps

### Local Development

1. **Start Local Validator**
```bash
# In a separate terminal
solana-test-validator

# In another terminal
anchor test
```

2. **Deploy Locally**
```bash
anchor deploy --provider.cluster localnet
```

### Devnet Deployment

1. **Configure for Devnet**
```bash
solana config set --url devnet
anchor build
```

2. **Deploy Program**
```bash
anchor deploy --provider.cluster devnet
```

3. **Verify Deployment**
```bash
solana program show <PROGRAM_ID>
```

### Mainnet Deployment (Production)

⚠️ **WARNING**: Only deploy to mainnet after thorough testing and security audits!

1. **Switch to Mainnet**
```bash
solana config set --url mainnet-beta
```

2. **Fund Deployment Account**
```bash
# You'll need real SOL for deployment fees
solana balance
```

3. **Deploy with Caution**
```bash
anchor deploy --provider.cluster mainnet-beta
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file:

```env
# Solana RPC URL
VITE_SOLANA_RPC_URL=https://api.devnet.solana.com

# Program ID (will be generated after deployment)
VITE_PROGRAM_ID=JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H

# Network
VITE_SOLANA_NETWORK=devnet
```

### Anchor Configuration

Update `anchor/Anchor.toml`:

```toml
[toolchain]
package_manager = "npm"

[features]
resolution = true
skip-lint = false

[programs.devnet]
cybervault = "YOUR_PROGRAM_ID_HERE"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"
```

## 🎮 Usage Instructions

### For Vault Owners

1. **Connect Wallet**
   - Use any Solana wallet (Phantom, Solflare, etc.)
   - Ensure you have SOL for transaction fees

2. **Initialize Vault**
   ```javascript
   await program.methods
     .initializeVault(beneficiaryPublicKey, timeoutSeconds)
     .accounts({...})
     .rpc();
   ```

3. **Deposit Tokens**
   ```javascript
   await program.methods
     .depositTokens(amount)
     .accounts({...})
     .rpc();
   ```

4. **Send Heartbeats**
   ```javascript
   await program.methods
     .sendHeartbeat()
     .accounts({...})
     .rpc();
   ```

### For Beneficiaries

1. **Monitor Vaults**
   - Check vault status regularly
   - Watch for timeout countdowns

2. **Claim Inheritance**
   ```javascript
   // Only after timeout expires
   await program.methods
     .claimInheritance()
     .accounts({...})
     .rpc();
   ```

## 🧪 Testing

### Unit Tests

```bash
# Run Anchor tests
npm run anchor-test

# Run specific test file
anchor test --skip-build tests/cybervault.test.ts
```

### Integration Tests

```bash
# Run full test suite
npm test

# Run frontend tests
npm run test:ui
```

### Demo Script

```bash
# Run the interactive demo
node demo.js
```

## 📊 Monitoring and Analytics

### Program Logs

```bash
# View program logs
solana logs <PROGRAM_ID>

# Monitor transactions
solana transaction-history <PROGRAM_ID> --limit 100
```

### Account Inspection

```bash
# Check program account
anchor account cybervault <VAULT_ADDRESS>

# View token accounts
spl-token accounts
```

## 🔒 Security Considerations

### Pre-Deployment Checklist

- [ ] Code has been audited by security professionals
- [ ] All tests pass with 100% coverage
- [ ] Timeout periods are reasonable (minimum 1 hour)
- [ ] Access controls are properly implemented
- [ ] PDA derivations are secure
- [ ] Token transfers are atomic
- [ ] Error handling is comprehensive

### Operational Security

- [ ] Use hardware wallets for mainnet
- [ ] Store private keys securely
- [ ] Monitor vault activity regularly
- [ ] Keep backup recovery phrases safe
- [ ] Test inheritance process thoroughly

### Smart Contract Risks

- [ ] **Re-entrancy**: Protected by Solana's execution model
- [ ] **Integer Overflow**: Using checked math operations
- [ ] **Access Control**: Role-based permissions implemented
- [ ] **Front-running**: Limited impact due to deterministic execution

## 🚨 Emergency Procedures

### If Something Goes Wrong

1. **Contract Issues**
   - Contact development team immediately
   - Document the issue with transaction signatures
   - DO NOT attempt to fix by sending more transactions

2. **Lost Access**
   - Use emergency withdrawal if you still have owner access
   - Contact beneficiary if timeout is approaching
   - Seek professional help for wallet recovery

3. **Network Issues**
   - Switch to different RPC endpoint
   - Check Solana network status
   - Wait for network stability before retrying

## 📈 Gas Optimization

### Transaction Costs

Typical transaction fees on Solana:

- Initialize Vault: ~0.001 SOL
- Deposit Tokens: ~0.0005 SOL
- Send Heartbeat: ~0.0002 SOL
- Claim Inheritance: ~0.0008 SOL
- Emergency Withdraw: ~0.0005 SOL

### Optimization Tips

- Batch operations when possible
- Use priority fees during high network usage
- Monitor SOL balance for transaction fees

## 🔄 Upgrading

### Program Upgrades

```bash
# Build new version
anchor build

# Upgrade program (if upgrade authority is set)
solana program deploy target/deploy/cybervault.so --program-id <PROGRAM_ID>
```

### Frontend Updates

```bash
# Update dependencies
npm update

# Regenerate client code after program changes
npm run codama:js

# Rebuild and deploy
npm run build
```

## 📚 Additional Resources

### Documentation
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework Guide](https://www.anchor-lang.com/)
- [SPL Token Program](https://spl.solana.com/token)

### Tools
- [Solana Explorer](https://explorer.solana.com/)
- [Anchor Explorer](https://anchor.so/)
- [SPL Token UI](https://token.solana.com/)

### Community
- [Solana Discord](https://discord.gg/solana)
- [Anchor Discord](https://discord.gg/anchor)
- [Cyber-Vault Community](https://discord.gg/cybervault)

## ❓ Troubleshooting

### Common Issues

**Program not found error**
```bash
# Solution: Ensure program is deployed and ID is correct
anchor keys sync
anchor deploy
```

**Transaction failed error**
```bash
# Solution: Check account balances and permissions
solana balance
spl-token accounts
```

**Timeout too short error**
```bash
# Solution: Use minimum 1 hour (3600 seconds)
const MINIMUM_TIMEOUT = 3600;
```

**Heartbeat failed error**
```bash
# Solution: Ensure you're the vault owner
// Check vault.owner == signer.publicKey
```

### Getting Help

1. Check the [Issues](https://github.com/your-org/cyber-vault/issues) page
2. Join our [Discord](https://discord.gg/cybervault) community
3. Read the [Documentation](https://docs.cyber-vault.xyz)
4. Contact the development team

## 🎉 Success!

If you've made it this far, you now have a fully deployed Cyber-Vault system! Your digital legacy is now protected by the immutable laws of mathematics and blockchain technology.

**Remember**: Code is Law, Math is Truth, Your Legacy is Eternal.

Welcome to the cyberpunk age of trustless digital inheritance! 🔐💀⚡

---

*For support and updates, visit [cyber-vault.xyz](https://cyber-vault.xyz)*