import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';
import { NotAuthorized } from 'features/auth/components';
import { ROUTES } from 'consts';

function NotAuthorizedPage() {
  const { account } = useAccount();

  if (!account) return <Navigate to={`/${ROUTES.LOGIN}`} />;

  return <NotAuthorized />;
}

export { NotAuthorizedPage };
