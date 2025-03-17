import '@gear-js/vara-ui/dist/style-deprecated.css';
import { useApi, useAccount } from '@gear-js/react-hooks';

import { Container, Footer } from '@dapps-frontend/ui';

import { ApiLoader } from '@/components';
import { Header } from '@/components/layout/header';
import { withProviders } from '@/hocs';
import { Routing } from '@/pages';

import './App.scss';

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
