import './global.css';
import './app.scss';
import { useApi, useAccount } from '@gear-js/react-hooks';

import { Container, Footer } from '@dapps-frontend/ui';

import { withProviders } from '@/app/hocs';
import { Header } from '@/components/layout';

import { ApiLoader } from './components/loaders/api-loader';
import { Routing } from './pages';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  return (
    <div className="flex flex-col min-h-screen">
      <Header />

      <main className="flex flex-col flex-1 container pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>

      <Container>
        <Footer />
      </Container>
    </div>
  );
};

export const App = withProviders(Component);
