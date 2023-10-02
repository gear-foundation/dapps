import { useAccount, useApi } from '@gear-js/react-hooks';
import { ApiLoader, Footer, Header, Loader } from 'components';
import { Routing } from 'pages';
import { withProviders } from 'hocs';
import { useNFTsState, useTestnetAutoLogin } from 'features/nfts';
import { useSearchParamsSetup } from 'features/node-switch';
import 'App.scss';
import { useTestnetNFTSetup } from './features/nfts/hooks';
import { usePendingUI } from './hooks';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const isAppReady = isApiReady && isAccountReady;
  // const ref = useRef<null | number>(null);

  useSearchParamsSetup();
  useTestnetAutoLogin();

  const { isPending } = usePendingUI();
  const isNFTStateReady = useNFTsState();
  const isTestnetStateReady = useTestnetNFTSetup();
  const isEachStateReady = isNFTStateReady && isTestnetStateReady && !isPending;

  // useEffect(() => {
  //   if (!ref.current) ref.current = performance.now();
  //
  //   if (isNFTStateReady && ref.current) {
  //     const diff = Math.floor((performance.now() - ref.current) * 1000) / 1_000_000;
  //     console.log(`${diff} seconds`);
  //     ref.current = null;
  //   }
  // }, [isNFTStateReady, isTestnetStateReady]);

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
