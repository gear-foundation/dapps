import './app.scss';
import '@gear-js/vara-ui/dist/style.css';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Routing } from './pages';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { withProviders } from '@/app/hocs';

import { useAccountAvailableBalanceSync, useWalletSync } from '@/features/wallet/hooks';
import { Container, Footer } from '@dapps-frontend/ui';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  useWalletSync();
  useAccountAvailableBalanceSync();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <main>
      {isAppReady ? (
        <>
          <Header />
          <Routing />
          <Container>
            <Footer vara isAlwaysMobile />
          </Container>
        </>
      ) : (
        <ApiLoader />
      )}
    </main>
  );
}

export const App = withProviders(Component);
