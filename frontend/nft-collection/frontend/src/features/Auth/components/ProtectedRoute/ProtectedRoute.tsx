import { Navigate, useLocation } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';
import { LOGIN, NOT_AUTHORIZED } from '@/routes';
import { AUTH_TOKEN_LOCAL_STORAGE_KEY } from '../../consts';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const authToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);
  const { account } = useAccount();
  const location = useLocation();

  if (!authToken && account) {
    return <Navigate to={NOT_AUTHORIZED} replace />;
  }

  if (!authToken) {
    return <Navigate to={LOGIN} state={{ from: location }} replace />;
  }

  return children;
}

export { ProtectedRoute };
