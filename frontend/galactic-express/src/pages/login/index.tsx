import { Navigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { WalletInfo } from 'features/wallet/components';
import { useAuth } from 'features/auth/hooks';
import { Welcome } from 'features/welcome/components/welcome';

function LoginPage() {
  const { authToken } = useAuth();
  const { account } = useAccount();

  if (authToken) {
    <Navigate to="/" replace />;
  }

  return (
    <Welcome>
      <WalletInfo account={account} />
    </Welcome>
  );
}

export { LoginPage };
