import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from 'error-tracking';
import { ROUTES } from 'consts';
import { Home } from './home';

export const routes = [{ path: ROUTES.HOME, Page: Home }];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
