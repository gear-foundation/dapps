import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';

import { ProtectedRouteProps } from './ProtectedRoute.interface';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { account } = useAccount();

  if (!account) {
    return <Navigate to="/" />;
  }

  return children;
}

export { ProtectedRoute };
