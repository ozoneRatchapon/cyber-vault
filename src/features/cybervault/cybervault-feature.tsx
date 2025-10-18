import { useState, useEffect } from 'react'
import { useSolana } from '@/components/solana/use-solana'
import { UiWalletAccount } from '@wallet-ui/react'
import { AppHero } from '@/components/app-hero'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Progress } from '@/components/ui/progress'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import {
  Shield,
  Heart,
  Skull,
  Coins,
  AlertTriangle,
  Zap,
  Lock,
  Unlock,
  Copy,
  CheckCircle,
  XCircle,
  Loader2,
  TrendingUp,
  Eye,
  RefreshCw,
} from 'lucide-react'
import { CybervaultUiProgramExplorerLink } from './ui/cybervault-ui-program-explorer-link'
import { useCybervaultMutations } from './data-access/use-cybervault-mutations'
import { useCyberVaultQuery, useVaultByOwnerQuery } from './data-access/use-cybervault-queries'
import { useUserTokensQuery, useTokenSelection, DEMO_TOKENS } from './data-access/use-token-selection'
import { toast } from 'sonner'
import { type Address } from 'gill'
import { WalletDropdown } from '@/components/wallet-dropdown'

export default function CybervaultFeature() {
  console.log('CybervaultFeature component rendering.');
  const { account } = useSolana();
  console.log('Account status:', account);

  if (!account) {
    console.log('No account connected. Displaying connect wallet prompt.');
    return (
      <div className="flex flex-col items-center justify-center min-h-screen">
        <h1 className="text-2xl font-bold mb-4">Welcome to CyberVault</h1>
        <p className="mb-8">Please connect your wallet to continue.</p>
        <WalletDropdown />
      </div>
    );
  }

  console.log('Account connected. Displaying main application content.');
  const [selectedTab, setSelectedTab] = useState<'create' | 'manage' | 'inherit'>('create')
  const [beneficiaryAddress, setBeneficiaryAddress] = useState('')
  const [timeoutHours, setTimeoutHours] = useState(24)
  const [depositAmount, setDepositAmount] = useState('')
  const [inheritOwnerAddress, setInheritOwnerAddress] = useState('')
  const [showAdvanced, setShowAdvanced] = useState(false)

  // Data hooks
  const vaultQuery = useCyberVaultQuery({ account: account ?? null })
  const inheritVaultQuery = useVaultByOwnerQuery({
    ownerAddress: (inheritOwnerAddress as Address | null),
  })
  const userTokensQuery = useUserTokensQuery({ account: account ?? null })
  const tokenSelection = useTokenSelection()

  // Mutation hooks
  const mutations = useCybervaultMutations({ account: account as UiWalletAccount })

  // Use demo tokens if no real tokens are available (for testing)
  const availableTokens = userTokensQuery.data?.length ? userTokensQuery.data : DEMO_TOKENS

  // Auto-select first token if none selected
  useEffect(() => {
    if (availableTokens.length > 0 && !tokenSelection.selectedToken && !tokenSelection.isCustomToken) {
      tokenSelection.selectToken(availableTokens[0])
    }
  }, [availableTokens, tokenSelection.selectedToken, tokenSelection.isCustomToken, tokenSelection])

  // Validation helpers
  const isValidSolanaAddress = (address: string): boolean => {
    try {
      return address.length >= 32 && address.length <= 44 && /^[1-9A-HJ-NP-Za-km-z]+$/.test(address)
    } catch {
      return false
    }
  }

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text)
    toast.success(`${label} copied to clipboard!`)
  }

  const formatAddress = (address: string, chars = 4): string => {
    return `${address.slice(0, chars)}...${address.slice(-chars)}`
  }

  const formatTimeRemaining = (seconds: number): string => {
    if (seconds <= 0) return 'EXPIRED'

    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = seconds % 60

    if (hours > 0) return `${hours}h ${minutes}m`
    if (minutes > 0) return `${minutes}m ${secs}s`
    return `${secs}s`
  }

  // Handle vault initialization
  const handleInitializeVault = async () => {
    if (!account?.address || !tokenSelection.getSelectedMint()) {
      toast.error('Missing required information')
      return
    }

    if (!isValidSolanaAddress(beneficiaryAddress)) {
      toast.error('Invalid beneficiary address')
      return
    }

    if (beneficiaryAddress === account.address) {
      toast.error('Cannot set yourself as beneficiary')
      return
    }

    if (timeoutHours < 1) {
      toast.error('Timeout must be at least 1 hour')
      return
    }

    try {
      await mutations.initializeVault.mutateAsync({
        beneficiary: beneficiaryAddress as Address,
        timeoutHours,
        mint: tokenSelection.getSelectedMint()!,
      })

      // Switch to manage tab after successful creation
      setSelectedTab('manage')
      setBeneficiaryAddress('')
      setTimeoutHours(24)
    } catch (error) {
      console.error('Initialize vault error:', error)
    }
  }

  // Handle token deposit
  const handleDepositTokens = async () => {
    if (!tokenSelection.getSelectedMint() || !depositAmount) {
      toast.error('Please select token and enter amount')
      return
    }

    const amount = parseFloat(depositAmount)
    if (isNaN(amount) || amount <= 0) {
      toast.error('Invalid deposit amount')
      return
    }

    const selectedToken = tokenSelection.selectedToken
    if (!selectedToken) return

    // Convert to smallest unit based on decimals
    const amountBN = BigInt(Math.floor(amount * 10 ** selectedToken.decimals))

    try {
      await mutations.depositTokens.mutateAsync({
        amount: amountBN,
        mint: tokenSelection.getSelectedMint()!,
      })

      setDepositAmount('')
    } catch (error) {
      console.error('Deposit tokens error:', error)
    }
  }

  // Handle inheritance claim
  const handleClaimInheritance = async () => {
    if (!inheritOwnerAddress) {
      toast.error('Please enter the owner address')
      return
    }

    if (!isValidSolanaAddress(inheritOwnerAddress)) {
      toast.error('Invalid owner address')
      return
    }

    try {
      await mutations.claimInheritance.mutateAsync({
        ownerAddress: inheritOwnerAddress as Address,
      })

      setInheritOwnerAddress('')
    } catch (error) {
      console.error('Claim inheritance error:', error)
    }
  }

  const vault = vaultQuery.data
  const inheritVault = inheritVaultQuery.data
  const isLoading = mutations.isAnyLoading || vaultQuery.isLoading
  const isInitialLoading = vaultQuery.isLoading && !vaultQuery.data

  return (
    <div className="max-w-6xl mx-auto p-6 space-y-6">
      <AppHero title="🔐 Cyber-Vault" subtitle="Decentralized Dead Man's Switch Protocol">
        <div className="flex items-center gap-4 text-sm opacity-75">
          <CybervaultUiProgramExplorerLink />
          <Badge variant="outline" className="border-purple-500 text-purple-400">
            {vault ? 'ACTIVE' : 'READY'}
          </Badge>
          {isLoading && <Loader2 className="h-4 w-4 animate-spin text-blue-400" />}
        </div>
      </AppHero>

      {/* Vault Status Overview */}
      {vault && (
        <Card className="border-green-500/20 bg-green-500/5">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5 text-green-400" />
              Vault Status Overview
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-green-400">{vault.totalDeposited.toString()}</div>
                <div className="text-sm text-muted-foreground">Total Secured</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-400">{formatTimeRemaining(vault.timeUntilTimeout)}</div>
                <div className="text-sm text-muted-foreground">Time Remaining</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-purple-400">{formatAddress(vault.beneficiary)}</div>
                <div className="text-sm text-muted-foreground">Beneficiary</div>
              </div>
              <div className="text-center">
                <Badge
                  variant={vault.isTimeoutReached ? 'destructive' : 'default'}
                  className={vault.isTimeoutReached ? 'bg-red-500/20 text-red-400' : 'bg-green-500/20 text-green-400'}
                >
                  {vault.isTimeoutReached ? 'CLAIMABLE' : 'PROTECTED'}
                </Badge>
              </div>
            </div>

            {!vault.isTimeoutReached && (
              <div className="mt-4">
                <div className="flex justify-between text-sm mb-2">
                  <span>Heartbeat Protection</span>
                  <span>
                    {Math.floor(
                      ((vault.timeoutSeconds * 1000 - (Date.now() - vault.lastHeartbeat.getTime())) /
                        (vault.timeoutSeconds * 1000)) *
                        100,
                    )}
                    %
                  </span>
                </div>
                <Progress
                  value={
                    ((vault.timeoutSeconds * 1000 - (Date.now() - vault.lastHeartbeat.getTime())) /
                      (vault.timeoutSeconds * 1000)) *
                    100
                  }
                  className="h-2"
                />
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-muted p-1 rounded-lg w-fit">
        <Button
          variant={selectedTab === 'create' ? 'default' : 'ghost'}
          onClick={() => setSelectedTab('create')}
          className="flex items-center gap-2"
          disabled={vault !== null} // Disable if vault already exists
        >
          <Lock className="h-4 w-4" />
          Create Vault
        </Button>
        <Button
          variant={selectedTab === 'manage' ? 'default' : 'ghost'}
          onClick={() => setSelectedTab('manage')}
          className="flex items-center gap-2"
          disabled={!vault} // Disable if no vault exists
        >
          <Heart className="h-4 w-4" />
          Manage Vault
        </Button>
        <Button
          variant={selectedTab === 'inherit' ? 'default' : 'ghost'}
          onClick={() => setSelectedTab('inherit')}
          className="flex items-center gap-2"
        >
          <Skull className="h-4 w-4" />
          Inherit Assets
        </Button>
      </div>

      {/* Create Vault Tab */}
      {selectedTab === 'create' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {isInitialLoading && (
            <div className="col-span-2 flex items-center justify-center py-12">
              <div className="flex items-center gap-3 text-muted-foreground">
                <Loader2 className="h-6 w-6 animate-spin" />
                <span>Loading vault data...</span>
              </div>
            </div>
          )}

          {vaultQuery.error && (
            <div className="col-span-2">
              <Alert className="border-red-500/20 bg-red-500/5">
                <AlertTriangle className="h-4 w-4 text-red-400" />
                <AlertDescription className="space-y-2">
                  <div className="font-semibold text-red-400">⚠️ Error Loading Vault Data</div>
                  <div className="text-sm">
                    There was an error loading your vault information. Please try refreshing the page.
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => vaultQuery.refetch()}
                    disabled={vaultQuery.isFetching}
                  >
                    {vaultQuery.isFetching ? (
                      <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    ) : (
                      <RefreshCw className="h-4 w-4 mr-2" />
                    )}
                    Retry
                  </Button>
                </AlertDescription>
              </Alert>
            </div>
          )}
          <Card className={`border-purple-500/20 ${vault ? 'opacity-50' : ''}`}>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5 text-purple-400" />
                Initialize Cyber-Vault
                {vault && <Badge variant="secondary">Already Created</Badge>}
              </CardTitle>
              <CardDescription>
                Create your decentralized dead man's switch with immutable smart contract protection
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {account == null ? (
                <Alert className="border-blue-500/20 bg-blue-500/5">
                  <AlertTriangle className="h-4 w-4 text-blue-400" />
                  <AlertDescription className="space-y-2">
                    <div className="font-semibold text-blue-400">🔗 Connect Your Wallet First</div>
                    <div className="text-sm">
                      To create a Cyber-Vault, you need to connect your Solana wallet.
                      Click the "Select Wallet" button in the top-right corner of the page.
                    </div>
                    <div className="text-xs text-muted-foreground">
                      💡 Don't have a wallet? Get one from{' '}
                      <a
                        href="https://solana.com/solana-wallets"
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-blue-400 hover:underline"
                      >
                        solana.com/solana-wallets
                      </a>
                    </div>
                  </AlertDescription>
                </Alert>
              ) : vault ? (
                <Alert>
                  <CheckCircle className="h-4 w-4" />
                  <AlertDescription>
                    You already have an active Cyber-Vault. Switch to the "Manage Vault" tab to interact with it.
                  </AlertDescription>
                </Alert>
              ) : (
                <>
                  {/* Token Selection */}
                  <div className="space-y-2">
                    <Label>Token to Secure</Label>
                    <Select
                      value={tokenSelection.isCustomToken ? 'custom' : tokenSelection.selectedToken?.mint || ''}
                      onValueChange={(value) => {
                        if (value === 'custom') {
                          setShowAdvanced(true)
                        } else {
                          const token = availableTokens.find((t) => t.mint === value)
                          if (token) tokenSelection.selectToken(token)
                        }
                      }}
                    >
                      <SelectTrigger>
                        <SelectValue placeholder="Select a token..." />
                      </SelectTrigger>
                      <SelectContent>
                        {availableTokens.map((token) => (
                          <SelectItem key={token.mint} value={token.mint}>
                            <div className="flex items-center gap-2">
                              <span className="font-mono text-xs">{token.symbol}</span>
                              <span className="text-muted-foreground">{token.name}</span>
                              <span className="text-xs text-muted-foreground">({token.uiAmount})</span>
                            </div>
                          </SelectItem>
                        ))}
                        <SelectItem value="custom">
                          <div className="flex items-center gap-2">
                            <Zap className="h-4 w-4" />
                            Custom Token...
                          </div>
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </div>

                  {/* Custom Token Input */}
                  {showAdvanced && (
                    <div className="space-y-2">
                      <Label htmlFor="customMint">Custom Token Mint Address</Label>
                      <Input
                        id="customMint"
                        placeholder="Enter mint address..."
                        value={tokenSelection.customMint}
                        onChange={(e) => {
                          tokenSelection.setCustomMint(e.target.value)
                        }}
                        onBlur={() => {
                          if (tokenSelection.customMint && tokenSelection.isValidMint(tokenSelection.customMint)) {
                            try {
                              tokenSelection.selectCustomToken(tokenSelection.customMint)
                            } catch (error) {
                              console.error('An error occurred:', error);
                              toast.error('Invalid mint address')
                            }
                          }
                        }}
                      />
                    </div>
                  )}

                  <div className="space-y-2">
                    <Label htmlFor="beneficiary">Beneficiary Address</Label>
                    <Input
                      id="beneficiary"
                      placeholder="Enter Solana wallet address..."
                      value={beneficiaryAddress}
                      onChange={(e) => setBeneficiaryAddress(e.target.value)}
                    />
                    <p className="text-xs text-muted-foreground">
                      The wallet that will inherit your assets after timeout
                    </p>
                  </div>

                  <div className="space-y-2">
                    <Label htmlFor="timeout">Timeout Period (Hours)</Label>
                    <Input
                      id="timeout"
                      type="number"
                      min="1"
                      value={timeoutHours}
                      onChange={(e) => setTimeoutHours(Number(e.target.value))}
                    />
                    <p className="text-xs text-muted-foreground">
                      Minimum 1 hour. Assets transfer after this period of digital silence.
                    </p>
                  </div>

                  <Alert>
                    <AlertTriangle className="h-4 w-4" />
                    <AlertDescription>
                      Once created, the beneficiary and timeout cannot be changed. Choose carefully.
                    </AlertDescription>
                  </Alert>

                  <Button
                    className="w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700"
                    onClick={handleInitializeVault}
                    disabled={
                      mutations.initializeVault.isPending ||
                      !tokenSelection.getSelectedMint() ||
                      !beneficiaryAddress ||
                      !isValidSolanaAddress(beneficiaryAddress) ||
                      timeoutHours < 1
                    }
                  >
                    {mutations.initializeVault.isPending ? (
                      <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    ) : (
                      <Lock className="h-4 w-4 mr-2" />
                    )}
                    Initialize Vault
                  </Button>
                </>
              )}
            </CardContent>
          </Card>

          <Card className="border-blue-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Zap className="h-5 w-5 text-blue-400" />
                How It Works
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex items-start gap-3">
                  <div className="bg-purple-500/20 rounded-full p-2">
                    <Shield className="h-4 w-4 text-purple-400" />
                  </div>
                  <div>
                    <h4 className="font-semibold">1. Initialize</h4>
                    <p className="text-sm text-muted-foreground">Create vault with beneficiary and timeout settings</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <div className="bg-green-500/20 rounded-full p-2">
                    <Coins className="h-4 w-4 text-green-400" />
                  </div>
                  <div>
                    <h4 className="font-semibold">2. Deposit</h4>
                    <p className="text-sm text-muted-foreground">Secure your SPL tokens in the blockchain vault</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <div className="bg-red-500/20 rounded-full p-2">
                    <Heart className="h-4 w-4 text-red-400" />
                  </div>
                  <div>
                    <h4 className="font-semibold">3. Heartbeat</h4>
                    <p className="text-sm text-muted-foreground">Send regular signals to prove you're alive</p>
                  </div>
                </div>

                <div className="flex items-start gap-3">
                  <div className="bg-orange-500/20 rounded-full p-2">
                    <Skull className="h-4 w-4 text-orange-400" />
                  </div>
                  <div>
                    <h4 className="font-semibold">4. Inherit</h4>
                    <p className="text-sm text-muted-foreground">Assets auto-transfer to beneficiary after timeout</p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Manage Vault Tab */}
      {selectedTab === 'manage' &&
        (!account ? (
          <Alert className="border-blue-500/20 bg-blue-500/5">
            <AlertTriangle className="h-4 w-4 text-blue-400" />
            <AlertDescription className="space-y-2">
              <div className="font-semibold text-blue-400">🔗 Connect Your Wallet</div>
              <div className="text-sm">
                Connect your Solana wallet to manage your Cyber-Vault.
                Click "Select Wallet" in the top-right corner.
              </div>
            </AlertDescription>
          </Alert>
        ) : vault ? (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <Card className="border-green-500/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Heart className="h-5 w-5 text-red-400" />
                  Vital Signs
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => vaultQuery.refetch()}
                    disabled={vaultQuery.isFetching}
                  >
                    <RefreshCw className={`h-4 w-4 ${vaultQuery.isFetching ? 'animate-spin' : ''}`} />
                  </Button>
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-sm">Status</span>
                  <Badge variant="outline" className="border-green-500 text-green-400">
                    <div className="w-2 h-2 bg-green-400 rounded-full mr-2 animate-pulse" />
                    {vault.isActive ? 'ACTIVE' : 'INACTIVE'}
                  </Badge>
                </div>

                <Separator />

                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Last Heartbeat</span>
                    <span className="text-sm font-mono">
                      {Math.floor((Date.now() - vault.lastHeartbeat.getTime()) / (1000 * 60 * 60))}h ago
                    </span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Time Until Timeout</span>
                    <span
                      className={`text-sm font-mono ${vault.isTimeoutReached ? 'text-red-400' : 'text-yellow-400'}`}
                    >
                      {formatTimeRemaining(vault.timeUntilTimeout)}
                    </span>
                  </div>
                </div>

                <Button
                  className="w-full bg-red-600 hover:bg-red-700"
                  onClick={() => mutations.sendHeartbeat.mutate()}
                  disabled={mutations.sendHeartbeat.isPending || vault.isTimeoutReached}
                >
                  {mutations.sendHeartbeat.isPending ? (
                    <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  ) : (
                    <Heart className="h-4 w-4 mr-2" />
                  )}
                  Send Heartbeat
                </Button>
              </CardContent>
            </Card>

            <Card className="border-purple-500/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Coins className="h-5 w-5 text-purple-400" />
                  Asset Management
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Total Secured</span>
                    <span className="text-sm font-mono">{vault.totalDeposited.toString()}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Token</span>
                    <span className="text-sm font-mono">{formatAddress(vault.mint)}</span>
                  </div>
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Vault Address</span>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => copyToClipboard(vault.tokenVault, 'Vault address')}
                    >
                      <Copy className="h-3 w-3" />
                    </Button>
                  </div>
                </div>

                <Separator />

                <div className="space-y-2">
                  <Label htmlFor="depositAmount">Deposit Amount</Label>
                  <Input
                    id="depositAmount"
                    type="number"
                    placeholder="0.00"
                    value={depositAmount}
                    onChange={(e) => setDepositAmount(e.target.value)}
                  />
                </div>

                <Button
                  className="w-full"
                  onClick={handleDepositTokens}
                  disabled={
                    mutations.depositTokens.isPending ||
                    !depositAmount ||
                    parseFloat(depositAmount) <= 0 ||
                    vault.isTimeoutReached
                  }
                >
                  {mutations.depositTokens.isPending ? (
                    <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  ) : (
                    <TrendingUp className="h-4 w-4 mr-2" />
                  )}
                  Deposit Tokens
                </Button>
              </CardContent>
            </Card>

            <Card className="border-yellow-500/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <AlertTriangle className="h-5 w-5 text-yellow-400" />
                  Emergency Actions
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-sm">Beneficiary</span>
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-mono">{formatAddress(vault.beneficiary)}</span>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => copyToClipboard(vault.beneficiary, 'Beneficiary address')}
                      >
                        <Copy className="h-3 w-3" />
                      </Button>
                    </div>
                  </div>
                </div>

                <Separator />

                <Dialog>
                  <DialogTrigger asChild>
                    <Button variant="destructive" className="w-full" disabled={vault.isTimeoutReached}>
                      <AlertTriangle className="h-4 w-4 mr-2" />
                      Emergency Withdraw
                    </Button>
                  </DialogTrigger>
                  <DialogContent>
                    <DialogHeader>
                      <DialogTitle>⚠️ Emergency Withdrawal</DialogTitle>
                      <DialogDescription>
                        This will withdraw all assets and deactivate your vault. This action cannot be undone.
                      </DialogDescription>
                    </DialogHeader>
                    <div className="flex gap-2 mt-4">
                      <Button
                        variant="destructive"
                        onClick={() => mutations.emergencyWithdraw.mutate()}
                        disabled={mutations.emergencyWithdraw.isPending}
                      >
                        {mutations.emergencyWithdraw.isPending ? (
                          <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        ) : (
                          <Unlock className="h-4 w-4 mr-2" />
                        )}
                        Confirm Withdrawal
                      </Button>
                    </div>
                  </DialogContent>
                </Dialog>
              </CardContent>
            </Card>
          </div>
        ) : (
          <Alert>
            <AlertTriangle className="h-4 w-4" />
            <AlertDescription>
              You do not have an active Cyber-Vault. Create one in the "Create Vault" tab.
            </AlertDescription>
          </Alert>
        ))}

      {/* Inherit Assets Tab */}
      {selectedTab === 'inherit' &&
        (!account ? (
          <Alert className="border-blue-500/20 bg-blue-500/5">
            <AlertTriangle className="h-4 w-4 text-blue-400" />
            <AlertDescription className="space-y-2">
              <div className="font-semibold text-blue-400">🔗 Connect Your Wallet</div>
              <div className="text-sm">
                Connect your Solana wallet to check for inheritable assets.
                Click "Select Wallet" in the top-right corner.
              </div>
            </AlertDescription>
          </Alert>
        ) : (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card className="border-orange-500/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Skull className="h-5 w-5 text-orange-400" />
                  Claim Digital Inheritance
                </CardTitle>
                <CardDescription>Check if you're entitled to inherit assets from a Cyber-Vault</CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="ownerAddress">Original Owner Address</Label>
                  <Input
                    id="ownerAddress"
                    placeholder="Enter the vault owner's address..."
                    value={inheritOwnerAddress}
                    onChange={(e) => setInheritOwnerAddress(e.target.value)}
                  />
                  <p className="text-xs text-muted-foreground">The address of the person who created the vault</p>
                </div>

                {inheritOwnerAddress && isValidSolanaAddress(inheritOwnerAddress) && (
                  <Button
                    variant="outline"
                    onClick={() => inheritVaultQuery.refetch()}
                    disabled={inheritVaultQuery.isFetching}
                    className="w-full"
                  >
                    {inheritVaultQuery.isFetching ? (
                      <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    ) : (
                      <Eye className="h-4 w-4 mr-2" />
                    )}
                    Check Vault Status
                  </Button>
                )}

                {inheritVault && (
                  <div className="space-y-2 p-3 border rounded-lg">
                    <div className="flex items-center justify-between">
                      <span className="text-sm">Vault Status</span>
                      <Badge variant={inheritVault.isActive ? 'default' : 'secondary'}>
                        {inheritVault.isActive ? 'Active' : 'Inactive'}
                      </Badge>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm">Beneficiary</span>
                      <span className="text-sm font-mono">{formatAddress(inheritVault.beneficiary)}</span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm">Assets</span>
                      <span className="text-sm font-mono">{inheritVault.totalDeposited.toString()}</span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-sm">Time Status</span>
                      <Badge variant={inheritVault.isTimeoutReached ? 'destructive' : 'default'}>
                        {inheritVault.isTimeoutReached
                          ? 'CLAIMABLE'
                          : formatTimeRemaining(inheritVault.timeUntilTimeout)}
                      </Badge>
                    </div>

                    {inheritVault.isTimeoutReached && inheritVault.beneficiary === account.address && (
                      <Button
                        className="w-full bg-gradient-to-r from-orange-600 to-red-600 hover:from-orange-700 hover:to-red-700"
                        onClick={handleClaimInheritance}
                        disabled={mutations.claimInheritance.isPending}
                      >
                        {mutations.claimInheritance.isPending ? (
                          <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        ) : (
                          <Skull className="h-4 w-4 mr-2" />
                        )}
                        Claim Inheritance
                      </Button>
                    )}

                    {inheritVault.beneficiary !== account.address && (
                      <Alert>
                        <XCircle className="h-4 w-4" />
                        <AlertDescription>You are not the designated beneficiary for this vault.</AlertDescription>
                      </Alert>
                    )}
                  </div>
                )}

                {inheritOwnerAddress && !inheritVault && !inheritVaultQuery.isFetching && (
                  <Alert>
                    <AlertTriangle className="h-4 w-4" />
                    <AlertDescription>No active vault found for this owner address.</AlertDescription>
                  </Alert>
                )}
              </CardContent>
            </Card>

            <Card className="border-gray-500/20">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <AlertTriangle className="h-5 w-5 text-yellow-400" />
                  Inheritance Protocol
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <Alert>
                    <Skull className="h-4 w-4" />
                    <AlertDescription>
                      <strong>Digital Death Detection:</strong> The silence of cyberspace confirms digital mortality.
                    </AlertDescription>
                  </Alert>

                  <div className="space-y-3">
                    <h4 className="font-semibold text-sm">Claiming Process:</h4>
                    <ol className="space-y-2 text-sm">
                      <li className="flex items-start gap-2">
                        <span className="bg-purple-500/20 text-purple-400 rounded-full w-5 h-5 flex items-center justify-center text-xs font-bold">
                          1
                        </span>
                        Verify timeout period has expired
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="bg-purple-500/20 text-purple-400 rounded-full w-5 h-5 flex items-center justify-center text-xs font-bold">
                          2
                        </span>
                        Confirm you are the designated beneficiary
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="bg-purple-500/20 text-purple-400 rounded-full w-5 h-5 flex items-center justify-center text-xs font-bold">
                          3
                        </span>
                        Execute inheritance claim transaction
                      </li>
                      <li className="flex items-start gap-2">
                        <span className="bg-purple-500/20 text-purple-400 rounded-full w-5 h-5 flex items-center justify-center text-xs font-bold">
                          4
                        </span>
                        Assets automatically transfer to your wallet
                      </li>
                    </ol>
                  </div>

                  <div className="bg-muted/50 p-3 rounded border-l-4 border-orange-500">
                    <p className="text-sm">
                      <strong>Code is Law:</strong> Once the timeout expires, the smart contract will automatically
                      execute the digital inheritance protocol. No human intervention required.
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        ))}

      {/* Footer */}
      <Card className="border-purple-500/20 bg-gradient-to-r from-purple-900/20 to-pink-900/20">
        <CardContent className="p-6">
          <div className="text-center space-y-2">
            <h3 className="text-lg font-semibold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
              Welcome to the Cyberpunk Age of Digital Inheritance
            </h3>
            <p className="text-sm text-muted-foreground">
              "Your digital legacy protected by mathematics, secured by immutable code"
            </p>
            <div className="flex items-center justify-center gap-4 text-xs opacity-60">
              <span>⚖️ Code is Law</span>
              <span>•</span>
              <span>🔒 Trustless Security</span>
              <span>•</span>
              <span>💀 Digital Immortality</span>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )

}
