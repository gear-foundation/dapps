import { useEffect } from 'react';
import { Route, Routes } from 'react-router-dom';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { AccountPage, CreateStreamPage, MainPage, StreamPage } from '@/pages';
import { Header, Footer } from '@/components';
import { withProviders } from '@/hocs';
import { ScrollToTop, cx } from '@/utils';
import { routes, CREATE_STREAM, ACCOUNT, STREAM } from '@/App.routes';
import { StreamTeasersList } from '@/features/StreamTeasers';
import { ProtectedRoute, AuthRoute } from '@/features/Auth/components';
import { Loader } from './components/Loader';
import styles from './App.module.scss';
import 'babel-polyfill';
import { useProgramState } from './hooks';
import { useCreateStreamMetadata, useGetStreamMetadata } from './features/CreateStream/hooks';
import { ADDRESS } from './consts';
import { useAccountAvailableBalanceSync } from './features/Wallet/hooks';

function AppComponent() {
  useCreateStreamMetadata();
  useAccountAvailableBalanceSync();
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isStateRead } = useProgramState();
  const { isMeta } = useGetStreamMetadata();

  const isAppReady = isApiReady && isAccountReady && isStateRead && isMeta;

  useEffect(() => {
    if (ADDRESS.CONTRACT) {
      console.log('CONTRACT ADDRESS:');
      console.log(ADDRESS.CONTRACT);
      console.log('NODE:');
      console.log(ADDRESS.NODE);
    }
  }, []);

  return (
    <div className={cx(styles['app-container'])}>
      <ScrollToTop />
      {isAppReady ? (
        <>
          <Header menu={routes} />
          <div className={cx(styles['main-content'])}>
            <Routes>
              <Route
                path="/"
                element={
                  <AuthRoute>
                    <MainPage />
                  </AuthRoute>
                }
              />
              <Route
                path={`/${ACCOUNT}`}
                element={
                  <ProtectedRoute>
                    <AccountPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={`/${CREATE_STREAM}`}
                element={
                  <ProtectedRoute>
                    <CreateStreamPage />
                  </ProtectedRoute>
                }
              />
              <Route
                path={`/${STREAM}`}
                element={
                  <ProtectedRoute>
                    <StreamPage />
                  </ProtectedRoute>
                }
              />
            </Routes>
          </div>
          <StreamTeasersList />
          <Footer />
        </>
      ) : (
        <Loader />
      )}
    </div>
  );
}

export const App = withProviders(AppComponent);
