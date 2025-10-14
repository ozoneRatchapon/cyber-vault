import { useState } from 'react'
import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { Shield, Heart, Skull, Timer, Coins, AlertTriangle, Zap, Lock, Unlock, Users, Clock } from 'lucide-react'
import { CybervaultUiProgramExplorerLink } from './ui/cybervault-ui-program-explorer-link'

interface VaultStatus {
  isActive: boolean
  totalDeposited: number
  lastHeartbeat: Date
  timeoutSeconds: number
  beneficiary: string
  owner: string
}

export default function CybervaultFeature() {
  const { account } = useSolana()
  const [selectedTab, setSelectedTab] = useState<'create' | 'manage' | 'inherit'>('create')
  const [beneficiaryAddress, setBeneficiaryAddress] = useState('')
  const [timeoutHours, setTimeoutHours] = useState(24)
  const [depositAmount, setDepositAmount] = useState('')
  const [vaultStatus] = useState<VaultStatus | null>(null)

  // Mock data for demonstration
  const mockVaultStatus: VaultStatus = {
    isActive: true,
    totalDeposited: 5000,
    lastHeartbeat: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
    timeoutSeconds: 24 * 60 * 60, // 24 hours
    beneficiary: 'Bene...4x7z',
    owner: 'Owner...8k2m',
  }

  const timeUntilTimeout = mockVaultStatus
    ? Math.max(0, mockVaultStatus.timeoutSeconds - (Date.now() - mockVaultStatus.lastHeartbeat.getTime()) / 1000)
    : 0
  const hoursUntilTimeout = Math.floor(timeUntilTimeout / 3600)
  const minutesUntilTimeout = Math.floor((timeUntilTimeout % 3600) / 60)

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <div className="max-w-md">
              <h1 className="text-5xl font-bold bg-gradient-to-r from-purple-400 via-pink-500 to-red-500 bg-clip-text text-transparent">
                🔐 CYBER-VAULT
              </h1>
              <p className="py-6 text-lg">Decentralized Dead Man's Switch for the Cyberpunk Age</p>
              <p className="text-sm opacity-75 mb-6">Protect your digital legacy with immutable smart contracts</p>
              <WalletDropdown />
            </div>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="max-w-6xl mx-auto p-6 space-y-6">
      <AppHero title="🔐 Cyber-Vault" subtitle="Decentralized Dead Man's Switch Protocol">
        <div className="flex items-center gap-4 text-sm opacity-75">
          <CybervaultUiProgramExplorerLink />
          <Badge variant="outline" className="border-purple-500 text-purple-400">
            PoC v1.0
          </Badge>
        </div>
      </AppHero>

      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-muted p-1 rounded-lg w-fit">
        <Button
          variant={selectedTab === 'create' ? 'default' : 'ghost'}
          onClick={() => setSelectedTab('create')}
          className="flex items-center gap-2"
        >
          <Lock className="h-4 w-4" />
          Create Vault
        </Button>
        <Button
          variant={selectedTab === 'manage' ? 'default' : 'ghost'}
          onClick={() => setSelectedTab('manage')}
          className="flex items-center gap-2"
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
          <Card className="border-purple-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5 text-purple-400" />
                Initialize Cyber-Vault
              </CardTitle>
              <CardDescription>
                Create your decentralized dead man's switch with immutable smart contract protection
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="beneficiary">Beneficiary Address</Label>
                <Input
                  id="beneficiary"
                  placeholder="Enter Solana wallet address..."
                  value={beneficiaryAddress}
                  onChange={(e) => setBeneficiaryAddress(e.target.value)}
                />
                <p className="text-xs text-muted-foreground">The wallet that will inherit your assets after timeout</p>
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

              <Button className="w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700">
                <Lock className="h-4 w-4 mr-2" />
                Initialize Vault
              </Button>
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
      {selectedTab === 'manage' && (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <Card className="border-green-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Heart className="h-5 w-5 text-red-400" />
                Vital Signs
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm">Status</span>
                <Badge variant="outline" className="border-green-500 text-green-400">
                  <div className="w-2 h-2 bg-green-400 rounded-full mr-2 animate-pulse" />
                  ACTIVE
                </Badge>
              </div>

              <Separator />

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm">Last Heartbeat</span>
                  <span className="text-sm font-mono">2h ago</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm">Time Until Timeout</span>
                  <span className="text-sm font-mono text-yellow-400">
                    {hoursUntilTimeout}h {minutesUntilTimeout}m
                  </span>
                </div>
              </div>

              <Button className="w-full bg-red-600 hover:bg-red-700">
                <Heart className="h-4 w-4 mr-2" />
                Send Heartbeat
              </Button>
            </CardContent>
          </Card>

          <Card className="border-blue-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Coins className="h-5 w-5 text-blue-400" />
                Digital Assets
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="text-center">
                <div className="text-3xl font-bold text-blue-400">5,000</div>
                <div className="text-sm text-muted-foreground">USDC Secured</div>
              </div>

              <Separator />

              <div className="space-y-2">
                <Label htmlFor="deposit">Deposit Amount</Label>
                <Input
                  id="deposit"
                  placeholder="Amount to deposit..."
                  value={depositAmount}
                  onChange={(e) => setDepositAmount(e.target.value)}
                />
              </div>

              <div className="grid grid-cols-2 gap-2">
                <Button variant="outline" size="sm">
                  <Coins className="h-4 w-4 mr-2" />
                  Deposit
                </Button>
                <Button variant="outline" size="sm">
                  <Unlock className="h-4 w-4 mr-2" />
                  Withdraw
                </Button>
              </div>
            </CardContent>
          </Card>

          <Card className="border-purple-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Users className="h-5 w-5 text-purple-400" />
                Vault Configuration
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm">Owner</span>
                  <code className="text-xs bg-muted px-2 py-1 rounded">{account.address.slice(0, 8)}...</code>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm">Beneficiary</span>
                  <code className="text-xs bg-muted px-2 py-1 rounded">Bene...4x7z</code>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm">Timeout</span>
                  <span className="text-sm">24 hours</span>
                </div>
              </div>

              <Alert>
                <Timer className="h-4 w-4" />
                <AlertDescription>
                  Configuration is immutable once set. Emergency withdrawal available.
                </AlertDescription>
              </Alert>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Inherit Assets Tab */}
      {selectedTab === 'inherit' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <Card className="border-orange-500/20">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Skull className="h-5 w-5 text-orange-400" />
                Digital Inheritance
              </CardTitle>
              <CardDescription>Claim inherited assets from cyber-vaults where you are the beneficiary</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Alert className="border-yellow-500/50 bg-yellow-500/10">
                <Clock className="h-4 w-4" />
                <AlertDescription>Inheritance can only be claimed after the timeout period expires</AlertDescription>
              </Alert>

              <div className="space-y-4">
                <div className="p-4 border rounded-lg bg-muted/50">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-semibold">Vault #1</span>
                    <Badge variant="outline" className="border-red-500 text-red-400">
                      TIMEOUT REACHED
                    </Badge>
                  </div>
                  <div className="text-sm space-y-1">
                    <div className="flex justify-between">
                      <span>Owner:</span>
                      <code className="text-xs">Dead...1234</code>
                    </div>
                    <div className="flex justify-between">
                      <span>Assets:</span>
                      <span className="font-mono">2,500 USDC</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Last Heartbeat:</span>
                      <span>25 hours ago</span>
                    </div>
                  </div>
                  <Button className="w-full mt-3 bg-gradient-to-r from-orange-600 to-red-600">
                    <Skull className="h-4 w-4 mr-2" />
                    Claim Inheritance
                  </Button>
                </div>

                <div className="p-4 border rounded-lg bg-muted/50 opacity-60">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-semibold">Vault #2</span>
                    <Badge variant="outline" className="border-green-500 text-green-400">
                      STILL ACTIVE
                    </Badge>
                  </div>
                  <div className="text-sm space-y-1">
                    <div className="flex justify-between">
                      <span>Owner:</span>
                      <code className="text-xs">Live...5678</code>
                    </div>
                    <div className="flex justify-between">
                      <span>Assets:</span>
                      <span className="font-mono">1,200 USDC</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Timeout in:</span>
                      <span>18 hours</span>
                    </div>
                  </div>
                  <Button disabled className="w-full mt-3">
                    <Timer className="h-4 w-4 mr-2" />
                    Waiting for Timeout
                  </Button>
                </div>
              </div>
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
                    <strong>Digital Death Detected:</strong> The silence of cyberspace has been confirmed.
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
      )}

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
