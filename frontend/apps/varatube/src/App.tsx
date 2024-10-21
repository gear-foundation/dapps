import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer } from '@dapps-frontend/ui';
import { Routing } from 'pages';
import { Header, ApiLoader } from 'components';
import { withProviders } from 'hocs';
import 'simplebar-react/dist/simplebar.min.css';
import 'App.scss';
import '@gear-js/vara-ui/dist/style.css';
import { useGetSubscriberQuery } from 'app/utils';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isFetched } = useGetSubscriberQuery();

  const isAppReady = isApiReady && isAccountReady && isFetched;

  return (
    <>
      <Header />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
