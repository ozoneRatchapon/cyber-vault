import { CYBERVAULT_PROGRAM_ADDRESS } from '@project/anchor'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { ellipsify } from '@wallet-ui/react'

export function CybervaultUiProgramExplorerLink() {
  return <AppExplorerLink address={CYBERVAULT_PROGRAM_ADDRESS} label={ellipsify(CYBERVAULT_PROGRAM_ADDRESS)} />
}
