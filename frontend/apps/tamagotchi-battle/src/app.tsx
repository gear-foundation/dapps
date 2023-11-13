import './app.scss';
import './index.css';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer } from 'ui';
import { Header } from 'components/layout';
import { ApiLoader } from 'components/loaders/api-loader';
import { useWalletSync } from 'features/wallet/hooks';
import { useAccountAvailableBalanceSync } from 'features/account-available-balance/hooks';
import { withProviders } from 'app/hocs';
import { Routing } from './pages';
import { useLocation } from 'react-router-dom';
import { ROUTES } from './app/consts';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { pathname } = useLocation();

  useWalletSync();
  useAccountAvailableBalanceSync();

  return (
    <div className="flex flex-col min-h-screen w-screen overflow-hidden">
      <Header />
      <main className="flex flex-col flex-1 smh:gap-1 gap-4 xxl:gap-8 pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>
      {pathname !== ROUTES.GAME && <Footer />}
    </div>
  );
};

export const App = withProviders(Component);
