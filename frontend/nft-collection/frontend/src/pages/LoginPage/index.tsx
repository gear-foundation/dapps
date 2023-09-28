import { Navigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { Welcome } from '@/features/Auth/components';
import { MAIN } from '@/routes';

function LoginPage() {
  const { account } = useAccount();

  if (account) {
    return <Navigate to={MAIN} replace />;
  }

  return (
    <Welcome>
      <WalletInfo account={account} />
    </Welcome>
  );
}

export { LoginPage };
