import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';
import { NotAuthorized } from '@/features/Auth/components';
import { LOGIN, MAIN } from '@/routes';
import { useAuth } from '@/features/Auth/hooks';

function NotAuthorizedPage() {
  const { account } = useAccount();
  const { authToken } = useAuth();

  if (!account) return <Navigate to={`/${LOGIN}`} />;
  if (account && authToken) return <Navigate to={MAIN} replace />;

  return <NotAuthorized />;
}

export { NotAuthorizedPage };
