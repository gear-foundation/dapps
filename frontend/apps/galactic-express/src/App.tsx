import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer } from '@dapps-frontend/ui';
import { Routing } from '@/pages';
import { Header, ApiLoader } from '@/components';
import { withProviders } from '@/hocs';
import '@gear-js/vara-ui/dist/style.css';
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
