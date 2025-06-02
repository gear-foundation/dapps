import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';

import { ACCOUNT } from '@/App.routes';

import { AuthRouteProps } from './AuthRoute.interface';

function AuthRoute({ children }: AuthRouteProps) {
  const { account } = useAccount();

  if (account) {
    return <Navigate to={ACCOUNT} />;
  }

  return children;
}

export { AuthRoute };
