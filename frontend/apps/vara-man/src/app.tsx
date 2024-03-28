import './global.css';
import './app.scss';

import { useApi, useAccount } from '@gear-js/react-hooks';
import { Container, Footer } from '@dapps-frontend/ui';
import { Routing } from './pages';
import { ApiLoader } from './components/loaders/api-loader';
import { Header } from '@/components/layout';
import { withProviders } from '@/app/hocs';

import '@gear-js/vara-ui/dist/style.css';


const Component = () => {

  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  return (
    <div className="flex flex-col min-h-screen overflow-hidden">
      <Header />
      <main className="flex flex-col flex-1 relative container pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>

      <Container className="z-1">
        <Footer vara />
      </Container>
    </div>
  );
};

export const App = withProviders(Component);
