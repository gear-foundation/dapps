import { useApi, useAccount } from '@gear-js/react-hooks';

import { ApiLoader } from '@/components';
import { withProviders } from '@/hocs';
import { Routing } from '@/pages';
import '@gear-js/vara-ui/dist/style.css';
import './App.scss';
import { Header } from '@/components/layout/header';

import { Container, Footer } from '@dapps-frontend/ui';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  return (
    <>
      <Header />
      <main>{isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}</main>
      <Container>
        <Footer vara />
      </Container>
    </>
  );
}

export const App = withProviders(Component);
