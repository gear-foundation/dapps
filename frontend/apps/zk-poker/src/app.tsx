import '@gear-js/vara-ui/dist/style-deprecated.css';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

import { withProviders } from '@/app/hocs';
import { ApiLoader } from '@/components';
import { Header } from '@/components/layout';
import { useAccountAvailableBalanceSync } from '@/features/wallet/hooks';

// import { useProgram } from './app/utils/sails';
import { ROUTES } from './app/consts';
import { Routing } from './pages';
import './app.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  // const program = useProgram();

  const page = useLocation().pathname;
  useAccountAvailableBalanceSync();

  const isAppReady = isApiReady && isAccountReady;
  // const isAppReady = isApiReady && isAccountReady && program;

  return (
    <main>
      {isAppReady ? (
        <>
          {page !== ROUTES.ONBOARDING && page !== ROUTES.LOGIN && <Header />}
          <Routing />
        </>
      ) : (
        <ApiLoader />
      )}
    </main>
  );
}

export const App = withProviders(Component);
