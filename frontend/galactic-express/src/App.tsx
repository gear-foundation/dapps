import { useApi, useAccount } from '@gear-js/react-hooks';
import { Routing } from 'pages';
import { Header, Footer, ApiLoader } from 'components';
import { withProviders } from 'hocs';
import 'App.scss';
import { useAuthSync } from 'features/auth/hooks';
import { useAccountAvailableBalance, useAccountAvailableBalanceSync, useWalletSync } from 'features/wallet/hooks';
import { useEffect } from 'react';
import { ADDRESS } from 'consts';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isAvailableBalanceReady } = useAccountAvailableBalance();

  const isAppReady = isApiReady && isAccountReady && isAvailableBalanceReady;

  useWalletSync();
  useAuthSync();
  useAccountAvailableBalanceSync();

  useEffect(() => {
    console.log('NODE:');
    console.log(ADDRESS.NODE);
  }, []);

  return (
    <>
      <Header />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
