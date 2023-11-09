import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from 'error-tracking';
import { ROUTES } from 'consts';
import { ProtectedRoute } from 'features/auth/components';
import { Home } from './home';
import { LoginPage } from './login';

export const routes = [
  { path: ROUTES.HOME, Page: Home, isProtected: true },
  { path: ROUTES.LOGIN, Page: LoginPage },
];

function Routing() {
  const getRoutes = () =>
    routes.map(({ path, Page, isProtected }) => (
      <Route
        key={path}
        path={path}
        element={
          isProtected ? (
            <ProtectedRoute>
              <Page />
            </ProtectedRoute>
          ) : (
            <Page />
          )
        }
      />
    ));

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
