import { Navigate } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { AuthRouteProps } from './AuthRoute.interface';
import { ACCOUNT } from '@/App.routes';

function AuthRoute({ children }: AuthRouteProps) {
  const { account } = useAccount();

  if (account) {
    return <Navigate to={ACCOUNT} />;
  }

  return children;
}

export { AuthRoute };
