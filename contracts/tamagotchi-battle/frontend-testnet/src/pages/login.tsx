import { Welcome } from '@/features/tic-tac-toe'
import { useAuth } from '@/features/auth'
import { Navigate } from 'react-router-dom'
import { ROUTES } from '@/app/consts'
import { Wallet } from '@/features/wallet'
import { useAccount } from '@gear-js/react-hooks'

export default function Login() {
  const { authToken } = useAuth()
  const { account } = useAccount()

  if (authToken) {
    return <Navigate to={ROUTES.HOME} replace />
  }

  return <Welcome>{!account && <Wallet account={account} isReady />}</Welcome>
}
