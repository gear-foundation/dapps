import { Navigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { Welcome } from '@/features/Main/components';
import { PLAY } from '@/App.routes';

function LoginPage() {
  const { account } = useAccount();

  if (account) {
    return <Navigate to={PLAY} replace />;
  }

  return (
    <Welcome>
      <WalletInfo account={account} />
    </Welcome>
  );
}

export { LoginPage };
