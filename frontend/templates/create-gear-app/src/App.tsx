import { useApi, useAccount } from '@gear-js/react-hooks';

import { Header, Footer, ApiLoader } from '@/components';
import { withProviders } from '@/hocs';
import { Routing } from '@/pages';
import './App.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <>
      <Header />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
