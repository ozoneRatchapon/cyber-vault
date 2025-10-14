import { useState, useEffect } from 'react'
import { useGetProgramAccountQuery } from '@/features/cybervault/data-access/use-get-program-account-query'
import { AppAlert } from '@/components/app-alert'
import { useSolana } from '@/components/solana/use-solana'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Shield, Code, Database, Activity, Zap, Lock, Heart, Timer, Skull, RefreshCw } from 'lucide-react'

interface ProgramStats {
  totalVaults: number
  activeVaults: number
  totalValueLocked: number
  heartbeatsToday: number
  inheritancesClaimed: number
}

export function CybervaultUiProgram() {
  const { cluster } = useSolana()
  const query = useGetProgramAccountQuery()
  const [stats, setStats] = useState<ProgramStats>({
    totalVaults: 0,
    activeVaults: 0,
    totalValueLocked: 0,
    heartbeatsToday: 0,
    inheritancesClaimed: 0,
  })

  // Simulate program statistics for demo
  useEffect(() => {
    const mockStats: ProgramStats = {
      totalVaults: 847,
      activeVaults: 823,
      totalValueLocked: 2547893.45,
      heartbeatsToday: 1247,
      inheritancesClaimed: 24,
    }
    setStats(mockStats)
  }, [])

  if (query.isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="text-center space-y-4">
          <RefreshCw className="h-8 w-8 animate-spin mx-auto text-purple-400" />
          <p className="text-sm text-muted-foreground">Connecting to Cyber-Vault Network...</p>
        </div>
      </div>
    )
  }

  if (!query.data?.value) {
    return (
      <AppAlert className="border-red-500/50 bg-red-500/10">
        <div className="flex items-center gap-2">
          <Skull className="h-4 w-4" />
          <div>
            <strong>Network Error:</strong> Cyber-Vault program not found on {cluster.label}.
            <br />
            <span className="text-sm">Deploy your program and reconnect to the matrix.</span>
          </div>
        </div>
      </AppAlert>
    )
  }

  return (
    <div className="space-y-6">
      {/* Program Header */}
      <Card className="border-purple-500/20 bg-gradient-to-r from-purple-900/10 to-pink-900/10">
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-6 w-6 text-purple-400" />
                Cyber-Vault Protocol Status
              </CardTitle>
              <CardDescription className="mt-1">
                Decentralized Dead Man's Switch Network on {cluster.label}
              </CardDescription>
            </div>
            <Badge variant="outline" className="border-green-500 text-green-400">
              <div className="w-2 h-2 bg-green-400 rounded-full mr-2 animate-pulse" />
              OPERATIONAL
            </Badge>
          </div>
        </CardHeader>
      </Card>

      {/* Network Statistics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="border-blue-500/20">
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Total Vaults</p>
                <p className="text-2xl font-bold text-blue-400">{stats.totalVaults.toLocaleString()}</p>
              </div>
              <Database className="h-8 w-8 text-blue-400/60" />
            </div>
          </CardContent>
        </Card>

        <Card className="border-green-500/20">
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Active Vaults</p>
                <p className="text-2xl font-bold text-green-400">{stats.activeVaults.toLocaleString()}</p>
              </div>
              <Activity className="h-8 w-8 text-green-400/60" />
            </div>
          </CardContent>
        </Card>

        <Card className="border-purple-500/20">
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Value Locked</p>
                <p className="text-2xl font-bold text-purple-400">${(stats.totalValueLocked / 1000000).toFixed(1)}M</p>
              </div>
              <Lock className="h-8 w-8 text-purple-400/60" />
            </div>
          </CardContent>
        </Card>

        <Card className="border-red-500/20">
          <CardContent className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-muted-foreground">Heartbeats Today</p>
                <p className="text-2xl font-bold text-red-400">{stats.heartbeatsToday.toLocaleString()}</p>
              </div>
              <Heart className="h-8 w-8 text-red-400/60" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Program Information */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card className="border-orange-500/20">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Code className="h-5 w-5 text-orange-400" />
              Smart Contract Details
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p className="text-muted-foreground">Program ID</p>
                <code className="text-xs bg-muted px-2 py-1 rounded break-all">
                  JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H
                </code>
              </div>
              <div>
                <p className="text-muted-foreground">Version</p>
                <p className="font-mono text-orange-400">v1.0.0</p>
              </div>
              <div>
                <p className="text-muted-foreground">Network</p>
                <p className="capitalize font-medium">{cluster.label}</p>
              </div>
              <div>
                <p className="text-muted-foreground">Language</p>
                <p className="font-medium">Rust + Anchor</p>
              </div>
            </div>

            <Separator />

            <div className="space-y-2">
              <h4 className="font-semibold text-sm">Available Instructions</h4>
              <div className="grid grid-cols-1 gap-2 text-sm">
                <div className="flex items-center gap-2 p-2 bg-muted/50 rounded">
                  <Shield className="h-4 w-4 text-blue-400" />
                  <code>initialize_vault</code>
                </div>
                <div className="flex items-center gap-2 p-2 bg-muted/50 rounded">
                  <Lock className="h-4 w-4 text-green-400" />
                  <code>deposit_tokens</code>
                </div>
                <div className="flex items-center gap-2 p-2 bg-muted/50 rounded">
                  <Heart className="h-4 w-4 text-red-400" />
                  <code>send_heartbeat</code>
                </div>
                <div className="flex items-center gap-2 p-2 bg-muted/50 rounded">
                  <Skull className="h-4 w-4 text-orange-400" />
                  <code>claim_inheritance</code>
                </div>
                <div className="flex items-center gap-2 p-2 bg-muted/50 rounded">
                  <Zap className="h-4 w-4 text-yellow-400" />
                  <code>emergency_withdraw</code>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="border-pink-500/20">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Timer className="h-5 w-5 text-pink-400" />
              Recent Network Activity
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-3">
              <div className="flex items-center gap-3 p-3 bg-green-500/10 rounded border border-green-500/20">
                <div className="w-2 h-2 bg-green-400 rounded-full animate-pulse" />
                <div className="flex-1">
                  <p className="text-sm font-medium">Vault Created</p>
                  <p className="text-xs text-muted-foreground">2 minutes ago</p>
                </div>
                <Badge variant="outline" className="text-xs border-green-500 text-green-400">
                  NEW
                </Badge>
              </div>

              <div className="flex items-center gap-3 p-3 bg-red-500/10 rounded border border-red-500/20">
                <div className="w-2 h-2 bg-red-400 rounded-full animate-pulse" />
                <div className="flex-1">
                  <p className="text-sm font-medium">Heartbeat Signal</p>
                  <p className="text-xs text-muted-foreground">5 minutes ago</p>
                </div>
                <Badge variant="outline" className="text-xs border-red-500 text-red-400">
                  💓
                </Badge>
              </div>

              <div className="flex items-center gap-3 p-3 bg-blue-500/10 rounded border border-blue-500/20">
                <div className="w-2 h-2 bg-blue-400 rounded-full animate-pulse" />
                <div className="flex-1">
                  <p className="text-sm font-medium">Token Deposit</p>
                  <p className="text-xs text-muted-foreground">12 minutes ago</p>
                </div>
                <Badge variant="outline" className="text-xs border-blue-500 text-blue-400">
                  💎
                </Badge>
              </div>

              <div className="flex items-center gap-3 p-3 bg-orange-500/10 rounded border border-orange-500/20">
                <div className="w-2 h-2 bg-orange-400 rounded-full" />
                <div className="flex-1">
                  <p className="text-sm font-medium">Inheritance Claimed</p>
                  <p className="text-xs text-muted-foreground">1 hour ago</p>
                </div>
                <Badge variant="outline" className="text-xs border-orange-500 text-orange-400">
                  💀
                </Badge>
              </div>
            </div>

            <Button variant="outline" size="sm" className="w-full border-pink-500/50 hover:bg-pink-500/10">
              <RefreshCw className="h-4 w-4 mr-2" />
              Refresh Activity
            </Button>
          </CardContent>
        </Card>
      </div>

      {/* Raw Program Data (for debugging) */}
      <Card className="border-gray-500/20">
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-sm">
            <Database className="h-4 w-4" />
            Raw Program Data
          </CardTitle>
          <CardDescription>Technical details for developers and auditors</CardDescription>
        </CardHeader>
        <CardContent>
          <pre className="text-xs bg-muted p-4 rounded overflow-x-auto">
            {JSON.stringify(query.data.value.data, null, 2)}
          </pre>
        </CardContent>
      </Card>

      {/* Footer Message */}
      <Card className="border-purple-500/20 bg-gradient-to-r from-purple-900/20 to-pink-900/20">
        <CardContent className="p-4 text-center">
          <p className="text-sm text-muted-foreground">
            🔮 <strong>Code is Law</strong> • Your digital legacy is secured by immutable mathematics •
            <strong>Welcome to the Future</strong> 💀
          </p>
        </CardContent>
      </Card>
    </div>
  )
}
