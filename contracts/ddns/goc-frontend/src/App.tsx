import { useApi, useAccount } from '@gear-js/react-hooks';
import { Header, Footer, ApiLoader, Content } from 'components';
import { withProviders } from 'hocs';
import { SUBHEADING } from 'consts';
import { Home } from 'pages';
import 'simplebar-react/dist/simplebar.min.css';
import 'App.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady, account } = useAccount();

  return (
    <>
      <Header isAccountVisible={isAccountReady} />
      <main>
        {isApiReady && isAccountReady ? (
          <>
            {account && <Home />}
            {!account && <Content subheading={SUBHEADING.LOGIN} />}
          </>
        ) : (
          <ApiLoader />
        )}
      </main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
