import { Navigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { useAuth } from '@/features/Auth/hooks';
import { Welcome } from '@/features/Auth/components';
import { MAIN } from '@/routes';

function LoginPage() {
  const { authToken } = useAuth();
  const { account } = useAccount();

  if (authToken) {
    return <Navigate to={MAIN} replace />;
  }

  return (
    <Welcome>
      <WalletInfo account={account} />
    </Welcome>
  );
}

export { LoginPage };
