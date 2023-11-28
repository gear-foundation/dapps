import { useEffect } from 'react';
import { Route, Routes } from 'react-router-dom';
import { useSetAtom } from 'jotai';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { useAuthSync } from '@/features/Auth/hooks';
import { MainPage } from '@/pages';
import { Header, Footer } from '@/components';
import { withProviders } from '@/hocs';
import { ScrollToTop, cx } from '@/utils';
import { LOGIN, NOT_AUTHORIZED } from '@/routes';
import { Loader } from './components/Loader';
import styles from './App.module.scss';
import 'babel-polyfill';
import { useFactoryState } from './hooks';
import { ACCOUNT_ATOM } from './atoms';
import { ProtectedRoute } from './features/Auth/components';
import { useWalletSync } from './features/Wallet/hooks';
import { LoginPage } from './pages/LoginPage';
import { NotAuthorizedPage } from './pages/NotAuthorizedPage';
import { COLLECTION, CREATE_COLLECTION, EXPLORE, MAIN, NFT, SEARCH, YOUR_SPACE, menu } from './routes';
import { ExplorePage } from './pages/ExplorePage';
import { YourSpacePage } from './pages/YourSpacePage';
import { CreateCollectionPage } from './pages/CreateCollectionPage';
import { CollectionPage } from './pages/CollectionPage';
import { NftPage } from './pages/NftPage';
import { useCollectionsState } from './features/Collection/hooks';
import { SearchPage } from './pages/SearchPage';

function AppComponent() {
  const { isApiReady } = useApi();
  const { isAccountReady, account } = useAccount();
  const { state, isStateRead } = useFactoryState();
  const { isCollectionsRead } = useCollectionsState();
  const setContractAccount = useSetAtom(ACCOUNT_ATOM);

  useEffect(() => {
    if (isStateRead && state && isAccountReady && account) {
      setContractAccount(account);
    }
  }, [state, isStateRead, account, isAccountReady, setContractAccount]);

  const isAppReady = isApiReady && isAccountReady && isStateRead && isCollectionsRead;

  useWalletSync();
  useAuthSync();

  return (
    <div className={cx(styles['app-container'])}>
      <ScrollToTop />
      {isAppReady ? (
        <>
          <Header menu={menu} />
          <div className={cx(styles['main-content'])}>
            <Routes>
              <Route
                path={MAIN}
                element={
                  <ProtectedRoute>
                    <MainPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={EXPLORE}
                element={
                  <ProtectedRoute>
                    <ExplorePage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={YOUR_SPACE}
                element={
                  <ProtectedRoute>
                    <YourSpacePage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={CREATE_COLLECTION}
                element={
                  <ProtectedRoute>
                    <CreateCollectionPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={`${COLLECTION}/:id`}
                element={
                  <ProtectedRoute>
                    <CollectionPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={`${NFT}/:collectionId/:nftId`}
                element={
                  <ProtectedRoute>
                    <NftPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={`${SEARCH}`}
                element={
                  <ProtectedRoute>
                    <SearchPage />
                  </ProtectedRoute>
                }
              />
              <Route path={`/${NOT_AUTHORIZED}`} element={<NotAuthorizedPage />} />
              <Route path={`/${LOGIN}`} element={<LoginPage />} />
            </Routes>
          </div>
          <Footer />
        </>
      ) : (
        <Loader />
      )}
    </div>
  );
}

export const App = withProviders(AppComponent);
