import { Navigate } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';
import { LOGIN } from '@/routes';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { account } = useAccount();

  if (!account) {
    return <Navigate to={`${LOGIN}`} />;
  }

  return children;
}

export { ProtectedRoute };
