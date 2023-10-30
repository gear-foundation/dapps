import { useAccountAvailableBalance } from '@/features/account-available-balance/hooks'
import { useAccount, useAlert } from '@gear-js/react-hooks'
import { stringShorten } from '@polkadot/util'
import { withoutCommas } from '../utils'

export function useCheckBalance() {
  const { account } = useAccount()
  const { availableBalance } = useAccountAvailableBalance()
  const alert = useAlert()

  const checkBalance = (payload: () => void, onError?: () => void) => {
    if (
      availableBalance &&
      Number(withoutCommas(availableBalance.value)) <
        Number(withoutCommas(availableBalance.existentialDeposit))
    ) {
      alert.error(
        `Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`
      )

      if (onError) {
        onError()
      }

      return
    }

    payload()
  }

  return { checkBalance }
}
