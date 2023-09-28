import { Navigate } from 'react-router';
import { ROUTES } from 'consts';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { account } = useAccount();

  if (!account) {
    return <Navigate to={`/${ROUTES.LOGIN}`} replace />;
  }

  return children;
}

export { ProtectedRoute };
