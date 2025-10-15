import { useQuery } from '@tanstack/react-query'
import { UiWalletAccount } from '@wallet-ui/react-gill'
import { fetchMaybeCyberVault, type CyberVault } from '@project/anchor'
import { createSolanaClient, type Address } from 'gill'
import { getVaultPDA } from './use-cybervault-mutations'

// Create RPC client for account fetching
const { rpc } = createSolanaClient({ urlOrMoniker: 'http://127.0.0.1:8899' })

export interface VaultStatus {
  isActive: boolean
  totalDeposited: bigint
  lastHeartbeat: Date
  timeoutSeconds: number
  beneficiary: string
  owner: string
  mint: string
  tokenVault: string
  isTimeoutReached: boolean
  timeUntilTimeout: number
  hoursUntilTimeout: number
  minutesUntilTimeout: number
}

export function useCyberVaultQuery({ account }: { account: UiWalletAccount | null }) {
  return useQuery({
    queryKey: ['cybervault', account?.address],
    queryFn: async (): Promise<VaultStatus | null> => {
      if (!account?.address) return null

      try {
        const vaultPDA = await getVaultPDA(account.address as Address)
        const vaultAccount = await fetchMaybeCyberVault(rpc, vaultPDA)

        if (!vaultAccount.exists) {
          return null
        }

        const vault = vaultAccount.data
        const lastHeartbeatMs = Number(vault.lastHeartbeat) * 1000
        const timeoutMs = Number(vault.timeoutSeconds) * 1000
        const now = Date.now()

        const timeUntilTimeout = Math.max(0, lastHeartbeatMs + timeoutMs - now)
        const isTimeoutReached = timeUntilTimeout === 0

        return {
          isActive: vault.isActive,
          totalDeposited: vault.totalDeposited,
          lastHeartbeat: new Date(lastHeartbeatMs),
          timeoutSeconds: Number(vault.timeoutSeconds),
          beneficiary: vault.beneficiary,
          owner: vault.owner,
          mint: vault.mint,
          tokenVault: vault.tokenVault,
          isTimeoutReached,
          timeUntilTimeout: Math.floor(timeUntilTimeout / 1000), // in seconds
          hoursUntilTimeout: Math.floor(timeUntilTimeout / (1000 * 60 * 60)),
          minutesUntilTimeout: Math.floor((timeUntilTimeout % (1000 * 60 * 60)) / (1000 * 60)),
        }
      } catch (error) {
        console.error('Error fetching vault:', error)
        return null
      }
    },
    enabled: !!account?.address,
    refetchInterval: 30000, // Refetch every 30 seconds to update timeout countdown
    staleTime: 10000, // Consider data stale after 10 seconds
  })
}

// Hook to check if user owns any vaults
export function useUserVaultsQuery({ account }: { account: UiWalletAccount | null }) {
  return useQuery({
    queryKey: ['cybervault', 'user-vaults', account?.address],
    queryFn: async (): Promise<CyberVault[]> => {
      if (!account?.address) return []

      try {
        // For now, we'll just check if the user has a vault as owner
        const vaultPDA = await getVaultPDA(account.address as Address)
        const vaultAccount = await fetchMaybeCyberVault(rpc, vaultPDA)

        if (vaultAccount.exists) {
          return [vaultAccount.data]
        }

        return []
      } catch (error) {
        console.error('Error fetching user vaults:', error)
        return []
      }
    },
    enabled: !!account?.address,
    staleTime: 30000, // Data is fresh for 30 seconds
  })
}

// Hook to check if user is a beneficiary of any vaults
export function useBeneficiaryVaultsQuery({ account }: { account: UiWalletAccount | null }) {
  return useQuery({
    queryKey: ['cybervault', 'beneficiary-vaults', account?.address],
    queryFn: async (): Promise<Address[]> => {
      if (!account?.address) return []

      try {
        // This would typically require a program-wide search or indexing
        // For now, we'll return empty array as this requires more complex querying
        // In a production app, you'd use program account filters or an indexer
        return []
      } catch (error) {
        console.error('Error fetching beneficiary vaults:', error)
        return []
      }
    },
    enabled: !!account?.address,
    staleTime: 60000, // Data is fresh for 60 seconds
  })
}

// Hook to fetch a specific vault by owner address
export function useVaultByOwnerQuery({ ownerAddress }: { ownerAddress: Address | null }) {
  return useQuery({
    queryKey: ['cybervault', 'by-owner', ownerAddress],
    queryFn: async (): Promise<VaultStatus | null> => {
      if (!ownerAddress) return null

      try {
        const vaultPDA = await getVaultPDA(ownerAddress)
        const vaultAccount = await fetchMaybeCyberVault(rpc, vaultPDA)

        if (!vaultAccount.exists) {
          return null
        }

        const vault = vaultAccount.data
        const lastHeartbeatMs = Number(vault.lastHeartbeat) * 1000
        const timeoutMs = Number(vault.timeoutSeconds) * 1000
        const now = Date.now()

        const timeUntilTimeout = Math.max(0, lastHeartbeatMs + timeoutMs - now)
        const isTimeoutReached = timeUntilTimeout === 0

        return {
          isActive: vault.isActive,
          totalDeposited: vault.totalDeposited,
          lastHeartbeat: new Date(lastHeartbeatMs),
          timeoutSeconds: Number(vault.timeoutSeconds),
          beneficiary: vault.beneficiary,
          owner: vault.owner,
          mint: vault.mint,
          tokenVault: vault.tokenVault,
          isTimeoutReached,
          timeUntilTimeout: Math.floor(timeUntilTimeout / 1000),
          hoursUntilTimeout: Math.floor(timeUntilTimeout / (1000 * 60 * 60)),
          minutesUntilTimeout: Math.floor((timeUntilTimeout % (1000 * 60 * 60)) / (1000 * 60)),
        }
      } catch (error) {
        console.error('Error fetching vault by owner:', error)
        return null
      }
    },
    enabled: !!ownerAddress,
    refetchInterval: 30000,
    staleTime: 10000,
  })
}
