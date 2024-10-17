import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer } from '@dapps-frontend/ui';
import { Routing } from 'pages';
import { Header, ApiLoader } from 'components';
import { withProviders } from 'hocs';
import { useProgramState } from 'hooks/api';
import 'simplebar-react/dist/simplebar.min.css';
import 'App.scss';
import '@gear-js/vara-ui/dist/style.css';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isSubscriptionsStateRead } = useProgramState();

  const isAppReady = isApiReady && isAccountReady && isSubscriptionsStateRead;

  return (
    <>
      <Header />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
