import { useEffect } from 'react';
import { Route, Routes } from 'react-router-dom';
import { useSetAtom } from 'jotai';
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
import { STREAM_TEASERS_ATOM, USERS_ATOM } from './atoms';

function AppComponent() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { state, isStateRead } = useProgramState();

  const setStreamTeasers = useSetAtom(STREAM_TEASERS_ATOM);
  const setUsers = useSetAtom(USERS_ATOM);

  useEffect(() => {
    if (state && isStateRead) {
      setStreamTeasers(state.streams);
      setUsers(state.users);
    }
  }, [state, isStateRead, setStreamTeasers, setUsers]);

  const isAppReady = isApiReady && isAccountReady && isStateRead;

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
