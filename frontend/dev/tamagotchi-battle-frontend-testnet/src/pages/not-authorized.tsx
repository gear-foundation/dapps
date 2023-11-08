import { useAccount } from '@gear-js/react-hooks'
import { Navigate } from 'react-router-dom'
import { ROUTES } from '@/app/consts'
import { NotAuthorized } from '@/components/layout/not-authorized'

export default function NotAuthorizedPage() {
  const { account } = useAccount()

  if (!account) return <Navigate to={ROUTES.LOGIN} replace />

  return <NotAuthorized />
}
