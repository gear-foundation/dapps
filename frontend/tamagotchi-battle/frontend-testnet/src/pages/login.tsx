import { useAuth } from '@/features/auth'
import { Navigate } from 'react-router-dom'
import { ROUTES } from '@/app/consts'
import { RegisterTamagotchi } from '@/features/battle-tamagotchi/register-tamagotchi'
import { useBattle } from '@/features/battle-tamagotchi/context'

export default function Login() {
  const { authToken } = useAuth()
  const { battle } = useBattle()

  if (authToken) {
    return <Navigate to={ROUTES.HOME} replace />
  }

  if (!battle) return null
  return <RegisterTamagotchi battle={battle} />
}
