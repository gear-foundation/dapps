import { Navigate } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { AuthRouteProps } from './AuthRoute.interface';

function AuthRoute({ children }: AuthRouteProps) {
  const { account } = useAccount();

  if (account) {
    return <Navigate to="/account" replace />;
  }

  return children;
}

export { AuthRoute };
