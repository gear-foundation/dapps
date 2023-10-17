import { useEffect } from 'react'
import { useSearchParams } from 'react-router-dom'
import { useAtom } from 'jotai'
import { useAccount, Account, useAlert } from '@gear-js/react-hooks'
import { useWallet } from 'features/wallet/hooks'
import { IS_AUTH_READY_ATOM, USER_ADDRESS_ATOM } from './atoms'
import { fetchAuth } from './utils'
import { CB_UUID_KEY } from './consts'
import { AuthResponse } from './types'

export function useAuth() {
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM)
  const [userAddress, setIsUserAddress] = useAtom(USER_ADDRESS_ATOM)
  const [query, setQuery] = useSearchParams()

  const { login, logout, account } = useAccount()
  const { resetWalletId } = useWallet()

  const resetSearchQuery = () => {
    query.delete('uuid')

    setQuery(query)
  }

  const signOut = () => {
    logout()
    resetWalletId()
    localStorage.removeItem(CB_UUID_KEY)
  }

  const auth = async () => {
    const uuid = query.get('uuid')
    const cbUuid = localStorage.getItem(CB_UUID_KEY)

    if (query.size && uuid) {
      localStorage.setItem(CB_UUID_KEY, uuid)
    }

    if (account) {
      try {
        const res = await fetchAuth<AuthResponse>('api/user/auth', 'POST', {
          coinbaseUID: uuid || cbUuid,
          substrate: account.decodedAddress,
        })

        if (res?.success) {
          setIsUserAddress(res.content.user.address)
        }

        if (!res?.success) {
          setIsUserAddress(null)
        }

        resetSearchQuery()
      } catch (err) {
        console.log(err)
      }
    }
    setIsAuthReady(true)
  }

  const signIn = async (_account: Account) => {
    await login(_account)
  }

  return { signIn, signOut, auth, isAuthReady, userAddress }
}

export function useAuthSync() {
  const { isAccountReady, account } = useAccount()
  const { auth } = useAuth()

  useEffect(() => {
    if (!isAccountReady) {
      return
    }

    auth()

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, account?.decodedAddress])
}

export function useAutoLogin() {
  const { login, accounts, isAccountReady } = useAccount()
  const alert = useAlert()

  const [searchParams, setSearchParams] = useSearchParams()

  useEffect(() => {
    if (!isAccountReady) return

    const accountAddress = searchParams.get('account')

    if (accountAddress) {
      const account = accounts.find(({ address }) => address === accountAddress)

      if (account) {
        login(account).then(() => {
          searchParams.delete('account')
          setSearchParams(searchParams)
        })
      } else {
        alert.error(`Account with address ${accountAddress} not found`)
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams, accounts, isAccountReady])
}
