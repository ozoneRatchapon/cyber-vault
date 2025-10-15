import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { UiWalletAccount, useWalletUiSignAndSend, useWalletUiSigner } from '@wallet-ui/react-gill'
import { toastTx } from '@/components/toast-tx'
import {
  getInitializeVaultInstructionAsync,
  getDepositTokensInstructionAsync,
  getSendHeartbeatInstructionAsync,
  getEmergencyWithdrawInstructionAsync,
  getClaimInheritanceInstruction,
  CYBERVAULT_PROGRAM_ADDRESS,
} from '@project/anchor'
import { getProgramDerivedAddress, getBytesEncoder, getAddressEncoder, type Address } from 'gill'

// Helper function to derive vault PDA
export async function getVaultPDA(owner: Address): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: CYBERVAULT_PROGRAM_ADDRESS,
    seeds: [
      getBytesEncoder().encode(new Uint8Array([99, 121, 98, 101, 114, 95, 118, 97, 117, 108, 116])), // "cyber_vault"
      getAddressEncoder().encode(owner),
    ],
  })
  return address
}

// Helper function to derive token vault PDA
export async function getTokenVaultPDA(owner: Address): Promise<Address> {
  const [address] = await getProgramDerivedAddress({
    programAddress: CYBERVAULT_PROGRAM_ADDRESS,
    seeds: [
      getBytesEncoder().encode(new Uint8Array([116, 111, 107, 101, 110, 95, 118, 97, 117, 108, 116])), // "token_vault"
      getAddressEncoder().encode(owner),
    ],
  })
  return address
}

export interface InitializeVaultArgs {
  beneficiary: Address
  timeoutHours: number
  mint: Address
}

export function useInitializeVaultMutation({ account }: { account: UiWalletAccount }) {
  const txSigner = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({ beneficiary, timeoutHours, mint }: InitializeVaultArgs) => {
      if (!account?.address) throw new Error('No wallet connected')

      const timeoutSeconds = timeoutHours * 3600 // Convert hours to seconds

      const instruction = await getInitializeVaultInstructionAsync({
        owner: txSigner,
        mint,
        beneficiary,
        timeoutSeconds,
      })

      return await signAndSend(instruction, txSigner)
    },
    onSuccess: (signature) => {
      toast.success('🔐 Cyber-Vault Initialized!', {
        description: 'Your digital legacy protection is now active',
      })
      toastTx(signature)
      // Invalidate vault queries to refetch data
      queryClient.invalidateQueries({ queryKey: ['cybervault'] })
    },
    onError: (error: any) => {
      console.error('Initialize vault error:', error)
      toast.error('Failed to Initialize Vault', {
        description: error?.message || 'An unexpected error occurred',
      })
    },
  })
}

export interface DepositTokensArgs {
  amount: bigint
  mint: Address
}

export function useDepositTokensMutation({ account }: { account: UiWalletAccount }) {
  const txSigner = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({ amount, mint }: DepositTokensArgs) => {
      if (!account?.address) throw new Error('No wallet connected')

      // Get token vault PDA
      const tokenVaultPDA = await getTokenVaultPDA(account.address as Address)

      const instruction = await getDepositTokensInstructionAsync({
        owner: txSigner,
        mint,
        ownerTokenAccount: account.address as Address, // This should be the user's token account
        tokenVault: tokenVaultPDA,
        amount,
      })

      return await signAndSend(instruction, txSigner)
    },
    onSuccess: (signature, variables) => {
      toast.success('💎 Assets Secured!', {
        description: `${variables.amount} tokens deposited to Cyber-Vault`,
      })
      toastTx(signature)
      queryClient.invalidateQueries({ queryKey: ['cybervault'] })
    },
    onError: (error: any) => {
      console.error('Deposit tokens error:', error)
      toast.error('Failed to Deposit Tokens', {
        description: error?.message || 'Transaction failed',
      })
    },
  })
}

export function useSendHeartbeatMutation({ account }: { account: UiWalletAccount }) {
  const txSigner = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      if (!account?.address) throw new Error('No wallet connected')

      const vaultPDA = await getVaultPDA(account.address as Address)

      const instruction = await getSendHeartbeatInstructionAsync({
        owner: txSigner,
        vault: vaultPDA,
      })

      return await signAndSend(instruction, txSigner)
    },
    onSuccess: (signature) => {
      toast.success('💓 Heartbeat Sent!', {
        description: "Digital presence confirmed - dead man's switch reset",
      })
      toastTx(signature)
      queryClient.invalidateQueries({ queryKey: ['cybervault'] })
    },
    onError: (error: any) => {
      console.error('Send heartbeat error:', error)
      toast.error('Failed to Send Heartbeat', {
        description: error?.message || 'Could not confirm digital presence',
      })
    },
  })
}

export function useEmergencyWithdrawMutation({ account }: { account: UiWalletAccount }) {
  const txSigner = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      if (!account?.address) throw new Error('No wallet connected')

      // This is a simplified version - in reality we'd need to pass the actual values
      const tokenVaultPDA = await getTokenVaultPDA(account.address as Address)

      const instruction = await getEmergencyWithdrawInstructionAsync({
        owner: txSigner,
        mint: 'So11111111111111111111111111111111111111112' as Address, // Placeholder
        ownerTokenAccount: account.address as Address,
        tokenVault: tokenVaultPDA,
        amount: 0n, // Placeholder - should be actual amount
      })

      return await signAndSend(instruction, txSigner)
    },
    onSuccess: (signature) => {
      toast.success('🚨 Emergency Withdrawal Complete!', {
        description: 'All assets recovered - digital sovereignty restored',
      })
      toastTx(signature)
      queryClient.invalidateQueries({ queryKey: ['cybervault'] })
    },
    onError: (error: any) => {
      console.error('Emergency withdraw error:', error)
      toast.error('Failed to Execute Emergency Withdrawal', {
        description: error?.message || 'Could not recover assets',
      })
    },
  })
}

export function useClaimInheritanceMutation({ account }: { account: UiWalletAccount }) {
  const txSigner = useWalletUiSigner({ account })
  const signAndSend = useWalletUiSignAndSend()
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({ ownerAddress }: { ownerAddress: Address }) => {
      if (!account?.address) throw new Error('No wallet connected')

      const vaultPDA = await getVaultPDA(ownerAddress)
      const tokenVaultPDA = await getTokenVaultPDA(ownerAddress)

      const instruction = getClaimInheritanceInstruction({
        beneficiary: txSigner,
        vault: vaultPDA,
        mint: 'So11111111111111111111111111111111111111112' as Address, // Placeholder
        tokenVault: tokenVaultPDA,
        beneficiaryTokenAccount: account.address as Address,
      })

      return await signAndSend(instruction, txSigner)
    },
    onSuccess: (signature) => {
      toast.success('⚰️ Inheritance Claimed!', {
        description: 'Digital assets transferred - the protocol has executed',
      })
      toastTx(signature)
      queryClient.invalidateQueries({ queryKey: ['cybervault'] })
    },
    onError: (error: any) => {
      console.error('Claim inheritance error:', error)
      toast.error('Failed to Claim Inheritance', {
        description: error?.message || 'Assets not yet available for claim',
      })
    },
  })
}

// Custom hook for multiple mutations with proper error boundaries
export function useCybervaultMutations({ account }: { account: UiWalletAccount }) {
  const initializeVault = useInitializeVaultMutation({ account })
  const depositTokens = useDepositTokensMutation({ account })
  const sendHeartbeat = useSendHeartbeatMutation({ account })
  const emergencyWithdraw = useEmergencyWithdrawMutation({ account })
  const claimInheritance = useClaimInheritanceMutation({ account })

  const isAnyLoading =
    initializeVault.isPending ||
    depositTokens.isPending ||
    sendHeartbeat.isPending ||
    emergencyWithdraw.isPending ||
    claimInheritance.isPending

  return {
    initializeVault,
    depositTokens,
    sendHeartbeat,
    emergencyWithdraw,
    claimInheritance,
    isAnyLoading,
  }
}
