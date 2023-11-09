import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from 'error-tracking';
import { Home } from './home';

const routes = [{ path: '/', Page: Home }];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
