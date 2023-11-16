import { Navigate } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { account } = useAccount();

  if (!account) {
    return <Navigate to="/" />;
  }

  return children;
}

export { ProtectedRoute };
