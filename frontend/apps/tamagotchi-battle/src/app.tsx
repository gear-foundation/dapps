import './app.scss';
import './index.css';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

import { Container, Footer } from '@dapps-frontend/ui';

import { withProviders } from '@/app/hocs';
import { Header } from '@/components/layout';
import { ApiLoader } from '@/components/loaders/api-loader';

import { ROUTES } from './app/consts';
import { Routing } from './pages';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { pathname } = useLocation();

  return (
    <div className="flex flex-col min-h-screen w-screen overflow-hidden">
      <Header />
      <main className="flex flex-col flex-1 smh:gap-1 gap-4 xxl:gap-8 pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>

      {pathname !== ROUTES.GAME && (
        <Container>
          <Footer />
        </Container>
      )}
    </div>
  );
};

export const App = withProviders(Component);
