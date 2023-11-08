import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';
import { ROUTES } from '@/app/consts';
import { NotAuthorized } from '@/components/layout/not-authorized';

export default function NotAuthorizedPage() {
  const { account } = useAccount();
  // const { authToken } = useAuth()
  if (!account) return <Navigate to={ROUTES.LOGIN} replace />;
  if (account) return <Navigate to={ROUTES.HOME} replace />;
  return <NotAuthorized />;
}
