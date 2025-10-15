import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { UiWalletAccount } from '@wallet-ui/react-gill'
import { type Address } from 'gill'

export interface TokenInfo {
  mint: Address
  symbol: string
  name: string
  decimals: number
  logoURI?: string
  balance: bigint
  uiAmount: string
}

// Common SPL tokens for testing/demo purposes
const COMMON_TOKENS: Record<string, Omit<TokenInfo, 'balance' | 'uiAmount'>> = {
  So11111111111111111111111111111111111111112: {
    mint: 'So11111111111111111111111111111111111111112' as Address,
    symbol: 'SOL',
    name: 'Wrapped SOL',
    decimals: 9,
    logoURI:
      'https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png',
  },
  EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v: {
    mint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' as Address,
    symbol: 'USDC',
    name: 'USD Coin',
    decimals: 6,
    logoURI:
      'https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v/logo.png',
  },
  Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB: {
    mint: 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB' as Address,
    symbol: 'USDT',
    name: 'Tether USD',
    decimals: 6,
    logoURI:
      'https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB/logo.png',
  },
}

// Hook to fetch user's token accounts
export function useUserTokensQuery({ account }: { account: UiWalletAccount | null }) {
  return useQuery({
    queryKey: ['user-tokens', account?.address],
    queryFn: async (): Promise<TokenInfo[]> => {
      if (!account?.address) return []

      try {
        // For now, return demo tokens as token account fetching needs more complex setup
        // In a real implementation, you would fetch actual token accounts from the blockchain
        return DEMO_TOKENS.map((token) => ({
          ...token,
          // Simulate some balance variations based on address
          balance: BigInt(Math.floor(Math.random() * 1000000000)),
          uiAmount: (Math.random() * 1000).toFixed(2),
        }))
      } catch (error) {
        console.error('Error fetching user tokens:', error)
        return DEMO_TOKENS
      }
    },
    enabled: !!account?.address,
    staleTime: 30000, // Cache for 30 seconds
    refetchInterval: 60000, // Refetch every minute
  })
}

// Hook for token selection with validation
export function useTokenSelection() {
  const [selectedToken, setSelectedToken] = useState<TokenInfo | null>(null)
  const [customMint, setCustomMint] = useState<string>('')
  const [isCustomToken, setIsCustomToken] = useState(false)

  // Validate custom mint address
  const isValidMint = (mint: string): boolean => {
    try {
      // Basic validation - should be base58 and proper length
      return mint.length >= 32 && mint.length <= 44 && /^[1-9A-HJ-NP-Za-km-z]+$/.test(mint)
    } catch {
      return false
    }
  }

  const selectToken = (token: TokenInfo) => {
    setSelectedToken(token)
    setIsCustomToken(false)
    setCustomMint('')
  }

  const selectCustomToken = (mintAddress: string) => {
    if (!isValidMint(mintAddress)) {
      throw new Error('Invalid mint address format')
    }

    setCustomMint(mintAddress)
    setIsCustomToken(true)
    setSelectedToken({
      mint: mintAddress as Address,
      symbol: 'CUSTOM',
      name: 'Custom Token',
      decimals: 0, // Would need to fetch from mint account
      balance: 0n,
      uiAmount: '0',
    })
  }

  const clearSelection = () => {
    setSelectedToken(null)
    setCustomMint('')
    setIsCustomToken(false)
  }

  const getSelectedMint = (): Address | null => {
    if (isCustomToken && customMint) {
      return customMint as Address
    }
    return selectedToken?.mint || null
  }

  const updateCustomMint = (value: string) => {
    setCustomMint(value)
  }

  return {
    selectedToken,
    customMint,
    isCustomToken,
    selectToken,
    selectCustomToken,
    setCustomMint: updateCustomMint,
    clearSelection,
    getSelectedMint,
    isValidMint,
  }
}

// Hook to get token info by mint address
export function useTokenInfoQuery({ mint }: { mint: Address | null }) {
  return useQuery({
    queryKey: ['token-info', mint],
    queryFn: async (): Promise<TokenInfo | null> => {
      if (!mint) return null

      try {
        // Check if it's a known token first
        const knownToken = COMMON_TOKENS[mint]
        if (knownToken) {
          return {
            ...knownToken,
            balance: 0n,
            uiAmount: '0',
          }
        }

        // For unknown tokens, we'd typically fetch mint account info
        // For now, return basic info
        return {
          mint,
          symbol: mint.slice(0, 4) + '...' + mint.slice(-4),
          name: 'Unknown Token',
          decimals: 0,
          balance: 0n,
          uiAmount: '0',
        }
      } catch (error) {
        console.error('Error fetching token info:', error)
        return null
      }
    },
    enabled: !!mint,
    staleTime: 300000, // Cache for 5 minutes (mint info doesn't change often)
  })
}

// Demo tokens for testing when no real tokens are available
export const DEMO_TOKENS: TokenInfo[] = [
  {
    mint: 'So11111111111111111111111111111111111111112' as Address,
    symbol: 'SOL',
    name: 'Wrapped SOL',
    decimals: 9,
    balance: 1000000000n, // 1 SOL
    uiAmount: '1.0',
    logoURI:
      'https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png',
  },
  {
    mint: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' as Address,
    symbol: 'USDC',
    name: 'USD Coin',
    decimals: 6,
    balance: 100000000n, // 100 USDC
    uiAmount: '100.0',
    logoURI:
      'https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v/logo.png',
  },
  {
    mint: 'DemoToken111111111111111111111111111111111' as Address,
    symbol: 'DEMO',
    name: 'Demo Token',
    decimals: 8,
    balance: 50000000000n, // 500 DEMO
    uiAmount: '500.0',
  },
]
