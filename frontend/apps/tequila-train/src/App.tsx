import '@gear-js/vara-ui/dist/style-deprecated.css';
import { useApi, useAccount } from '@gear-js/react-hooks';

import { Footer, Container } from '@dapps-frontend/ui';

import { withProviders } from '@/app/hocs';
import { Header } from '@/components/layout';
import { ApiLoader } from '@/components/loaders/api-loader';
import { Routing } from '@/pages';

import './index.css';
import './App.scss';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1">{isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}</main>

      <Container>
        <Footer vara />
      </Container>
    </div>
  );
};

export const App = withProviders(Component);
