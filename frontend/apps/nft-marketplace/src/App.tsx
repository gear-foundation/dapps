import { useApi, useAccount } from '@gear-js/react-hooks';

import { Footer } from '@dapps-frontend/ui';

import { Header, ApiLoader } from '@/components';
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
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
