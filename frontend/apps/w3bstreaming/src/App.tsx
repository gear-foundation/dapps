import { useAccount, useApi } from '@gear-js/react-hooks';
import { Route, Routes } from 'react-router-dom';

import { routes, CREATE_STREAM, ACCOUNT, STREAM } from '@/App.routes';
import { Header, Footer } from '@/components';
import { ProtectedRoute, AuthRoute } from '@/features/Auth/components';
import { StreamTeasersList } from '@/features/StreamTeasers';
import { withProviders } from '@/hocs';
import { AccountPage, CreateStreamPage, MainPage, StreamPage } from '@/pages';
import { useScrollToTop, cx } from '@/utils';

import styles from './App.module.scss';
import { Loader } from './components/Loader';
import { useAccountAvailableBalanceSync } from './features/Wallet/hooks';
import '@gear-js/vara-ui/dist/style.css';
import { useGetStateQuery } from './app/utils';

function AppComponent() {
  useScrollToTop();
  useAccountAvailableBalanceSync();

  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isFetched } = useGetStateQuery();

  const isAppReady = isApiReady && isAccountReady && isFetched;

  return (
    <div className={cx(styles['app-container'])}>
      {isAppReady ? (
        <>
          <Header menu={routes} />
          <div className={cx(styles['main-content'])}>
            <div className={cx(styles['wrapped-content'])}>
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
            <div className={cx(styles['teasers-list-wrapper'])}>
              <StreamTeasersList />
            </div>
          </div>
          <div className={cx(styles['footer-wrapper'])}>
            <Footer />
          </div>
        </>
      ) : (
        <Loader />
      )}
    </div>
  );
}

export const App = withProviders(AppComponent);
