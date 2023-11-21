import { useAtom } from 'jotai'
import { useAlert, Account, useAccount } from '@gear-js/react-hooks'
import { useEffect } from 'react'
import { web3FromAddress } from '@polkadot/extension-dapp'
import {
  AUTH_MESSAGE,
  AUTH_TOKEN_ATOM,
  AUTH_TOKEN_LOCAL_STORAGE_KEY,
} from './consts'
import { fetchAuth, post } from './utils'
import { AuthResponse, ISignInError, SignInResponse } from './types'
import { useLocation, useNavigate } from 'react-router-dom'
import { ROUTES } from '@/app/consts'

export function useAuth() {
  const { login, logout } = useAccount()
  const alert = useAlert()
  const [authToken, setAuthToken] = useAtom(AUTH_TOKEN_ATOM)

  const navigate = useNavigate()
  const location = useLocation()
  const from = location.state?.from?.pathname || ROUTES.HOME

  const signIn = async (account: Account) => {
    const { address } = account

    try {
      const { signer } = await web3FromAddress(address)
      if (!signer.signRaw) throw new Error('signRaw not exists')

      const { signature } = await signer.signRaw({
        address,
        data: AUTH_MESSAGE,
        type: 'payload',
      })
      const res = await post('auth/login', {
        signature,
        publicKey: address,
        message: AUTH_MESSAGE,
      })

      if (!res.ok) {
        const data: ISignInError = await res.json()
        alert.error(
          data.message
            ? data.message
            : data.errors
            ? data.errors.message
            : 'Something wrong'
        )
        setAuthToken(null)
        await login(account)
        navigate(ROUTES.UNAUTHORIZED, { replace: true })
      } else {
        const data: SignInResponse = await res.json()
        const { accessToken } = data

        await login(account)
        setAuthToken(accessToken)
        navigate(from, { replace: true })
      }
    } catch (e) {
      alert.error(`${e}`)
    }
  }

  const signOut = () => {
    logout()
    setAuthToken(null)
  }

  const auth = () => {
    if (!authToken) return

    fetchAuth<AuthResponse>('auth/me', 'PUT', authToken).catch(
      ({ message }: Error) => {
        signOut()
        alert.error(message)
      }
    )
  }

  return { authToken, signIn, signOut, auth }
}

export function useAuthSync() {
  const { authToken, auth } = useAuth()

  useEffect(() => {
    auth()
  }, [authToken])

  useEffect(() => {
    if (!authToken) return localStorage.removeItem(AUTH_TOKEN_LOCAL_STORAGE_KEY)

    localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken)
  }, [authToken])
}
