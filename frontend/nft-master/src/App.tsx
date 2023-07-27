import { useApi, useAccount } from '@gear-js/react-hooks';
import { Header, Footer, ApiLoader, Loader } from 'components';
import { Routing } from 'pages';
import { withProviders } from 'hocs';
import { useNFTsState, useTestnetAutoLogin } from 'features/nfts';
import { useSearchParamsSetup } from 'features/node-switch';
import 'App.scss';
import { useTestnetNFTSetup } from 'features/nfts/hooks';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const isAppReady = isApiReady && isAccountReady;

  useSearchParamsSetup();
  useTestnetAutoLogin();

  const isNFTsStateReady = useNFTsState();
  const isTestnetNFTStateReady = useTestnetNFTSetup();

  const isEachStateReady = isNFTsStateReady && isTestnetNFTStateReady;

  return (
    <>
      <Header />
      <main>
        {isAppReady ? (
          <>
            {isEachStateReady && <Routing />}
            {!isEachStateReady && <Loader />}
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
