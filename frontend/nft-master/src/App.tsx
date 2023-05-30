import { useApi, useAccount } from '@gear-js/react-hooks';
import { useLayoutEffect } from 'react';
import { Header, Footer, ApiLoader } from 'components';
import { Routing } from 'pages';
import { withProviders } from 'hocs';
import { useContractAddress } from 'features/contract-address';
import 'App.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const isAppReady = isApiReady && isAccountReady;

  const contractAddress = useContractAddress();

  useLayoutEffect(() => {
    if (contractAddress) document.body.classList.add('contract');
  }, [contractAddress]);

  return (
    <>
      <Header />
      <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </>
  );
}

export const App = withProviders(Component);
