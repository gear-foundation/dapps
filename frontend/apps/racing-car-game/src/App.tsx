import { Route, Routes } from 'react-router-dom';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { Container, Footer } from '@dapps-frontend/ui';
import { GamePage, MainPage } from '@/pages';
import { Header } from '@/components';
import { withProviders } from '@/hocs';
import { ScrollToTop, cx } from '@/utils';
import { LOGIN, PLAY, START } from '@/App.routes';
import styles from './App.module.scss';
import { useLoginByParams } from './hooks';
import { ProtectedRoute } from './features/Auth/components';
import { useAccountAvailableBalance, useAccountAvailableBalanceSync } from './features/Wallet/hooks';
import { LoginPage } from './pages/LoginPage';
import { ApiLoader } from './components/ApiLoader';
import { useAuth, useAuthSync } from './features/Auth/hooks';
import '@gear-js/vara-ui/dist/style.css';

function AppComponent() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isAvailableBalanceReady } = useAccountAvailableBalance();
  const { isAuthReady } = useAuth();

  const isAppReady = isApiReady && isAccountReady && isAvailableBalanceReady && isAuthReady;

  useAuthSync();
  useLoginByParams();
  useAccountAvailableBalanceSync();

  return (
    <div className={cx(styles['app-container'])}>
      <ScrollToTop />
      {isAppReady ? (
        <>
          <Header />

          <ErrorTrackingRoutes>
            <Route
              path="*"
              element={
                <>
                  <div className={cx(styles['main-content'])}>
                    <Routes>
                      <Route
                        index
                        path={`/${PLAY}`}
                        element={
                          <ProtectedRoute>
                            <MainPage />
                          </ProtectedRoute>
                        }
                      />
                      <Route path={`/${LOGIN}`} element={<LoginPage />} />
                    </Routes>
                  </div>

                  <Container>
                    <Footer vara />
                  </Container>
                </>
              }
            />

            <Route
              path={`/${START}`}
              element={
                <div className={cx(styles['main-content'])}>
                  <ProtectedRoute>
                    <GamePage />
                  </ProtectedRoute>
                </div>
              }
            />
          </ErrorTrackingRoutes>
        </>
      ) : (
        <ApiLoader />
      )}
    </div>
  );
}

export const App = withProviders(AppComponent);
