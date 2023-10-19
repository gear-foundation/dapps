import { atom, useAtomValue, useSetAtom } from 'jotai'
import { useAccount, useApi } from '@gear-js/react-hooks'
import { useEffect } from 'react'
import {
  useAccountAvailableBalance,
  useAccountAvailableBalanceSync,
} from 'features/available-balance/hooks'
import { useAuth } from 'features/auth/hooks'

const isAppReadyAtom = atom<boolean>(false)

export function useIsAppReady() {
  const isAppReady = useAtomValue(isAppReadyAtom)
  const setIsAppReady = useSetAtom(isAppReadyAtom)

  return { isAppReady, setIsAppReady }
}

export function useIsAppReadySync() {
  const { isApiReady } = useApi()
  const { isAccountReady } = useAccount()
  const { isAvailableBalanceReady } = useAccountAvailableBalance()
  const { isAuthReady } = useAuth()

  const { setIsAppReady } = useIsAppReady()

  useAccountAvailableBalanceSync()

  useEffect(() => {
    setIsAppReady(
      isApiReady && isAccountReady && isAvailableBalanceReady && isAuthReady
    )
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, isApiReady, isAvailableBalanceReady, isAuthReady])
}
