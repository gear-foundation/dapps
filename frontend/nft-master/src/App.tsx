import { useApi, useAccount, useAlert } from '@gear-js/react-hooks';
import { Header, Footer, ApiLoader, Loader } from 'components';
import { Routing } from 'pages';
import { withProviders } from 'hocs';
import { useNFTsState } from 'features/nfts';
import { useContractAddressSetup } from 'features/contract-address';
import 'App.scss';
import { useEffect } from 'react';

function Component() {
  const alert = useAlert();
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const isAppReady = isApiReady && isAccountReady;

  useContractAddressSetup();
  const isNFTsStateReady = useNFTsState();

  useEffect(() => {
    alert.error('Test', { timeout: 1000000 });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
