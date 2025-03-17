import '@gear-js/vara-ui/dist/style-deprecated.css';
import { useAccount, useApi } from '@gear-js/react-hooks';

import { Container, Footer } from '@dapps-frontend/ui';

import { withProviders } from '@/app/hocs';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { useAccountAvailableBalanceSync } from '@/features/wallet/hooks';

import { useProgram } from './app/utils/sails';
import { Routing } from './pages';
import './app.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const program = useProgram();

  useAccountAvailableBalanceSync();

  const isAppReady = isApiReady && isAccountReady && program;

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
