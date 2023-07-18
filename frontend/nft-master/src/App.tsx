import { useApi, useAccount } from '@gear-js/react-hooks';
import { Header, Footer, ApiLoader, Loader } from 'components';
import { Routing } from 'pages';
import { withProviders } from 'hocs';
import { useNFTsState } from 'features/nfts';
import { useContractAddressSetup } from 'features/contract-address';
import 'App.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const isAppReady = isApiReady && isAccountReady;

  useContractAddressSetup();
  const isNFTsStateReady = useNFTsState();

  return (
    <>
      <Header />
      <main>
        {isAppReady ? (
          <>
            {isNFTsStateReady && <Routing />}
            {!isNFTsStateReady && <Loader />}
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
