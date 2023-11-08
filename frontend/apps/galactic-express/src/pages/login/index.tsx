import { Navigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { WalletInfo } from 'features/wallet/components';
import { Welcome } from 'features/welcome/components/welcome';

function LoginPage() {
  const { account } = useAccount();

  if (account) {
    <Navigate to="/" replace />;
  }

  return (
    <Welcome>
      <WalletInfo account={account} />
    </Welcome>
  );
}

export { LoginPage };
