import { useAccount } from '@gear-js/react-hooks';
import { Navigate } from 'react-router-dom';

import { PLAY } from '@/App.routes';
import { Welcome } from '@/features/Main/components';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';

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
