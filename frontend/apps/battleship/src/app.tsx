import './app.scss';
import '@gear-js/vara-ui/dist/style.css';
import { useAccount, useApi } from '@gear-js/react-hooks';

import { Container, Footer } from '@dapps-frontend/ui';

import { withProviders } from '@/app/hocs';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { useAccountAvailableBalanceSync } from '@/features/wallet/hooks';

import { Routing } from './pages';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

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
