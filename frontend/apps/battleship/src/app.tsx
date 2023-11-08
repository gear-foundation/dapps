import './app.scss';
import '@gear-js/vara-ui/dist/style.css';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Routing } from './pages';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { withProviders } from '@/app/hocs';

import { useWalletSync } from '@/features/wallet/hooks';
import { ADDRESS } from '@/app/consts';
import { useEffect } from 'react';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  useWalletSync();

  const isAppReady = isApiReady && isAccountReady;

  useEffect(() => {
    console.log('ADDRESS NODE:', ADDRESS.NODE);
    console.log('ADDRESS GAME:', ADDRESS.GAME);
  }, []);

  return (
    <main>
      {isAppReady ? (
        <>
          <Header />
          <Routing />
        </>
      ) : (
        <ApiLoader />
      )}
    </main>
  );
}

export const App = withProviders(Component);
