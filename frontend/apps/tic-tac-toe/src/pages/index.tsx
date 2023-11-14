import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { ROUTES } from '@/app/consts';
import Home from '@/pages/home';
import NotFoundPage from '@/pages/not-found';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.NOTFOUND, Page: NotFoundPage },
];

export function Routing() {
  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </ErrorTrackingRoutes>
  );
}
