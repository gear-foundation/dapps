import { Navigate } from 'react-router';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';
import { LOGIN } from '@/App.routes';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { account } = useAccount();

  if (!account) {
    return <Navigate to={`/${LOGIN}`} replace />;
  }

  return children;
}

export { ProtectedRoute };
