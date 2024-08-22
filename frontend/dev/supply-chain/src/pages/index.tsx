import { useAccount } from '@gear-js/react-hooks';
import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { Content } from 'components';
import { Home } from './home';

const routes = [{ path: '/', Page: Home, isPrivate: true }];

function Routing() {
  const { account } = useAccount();

  const getRoutes = () =>
    routes.map(({ path, Page, isPrivate }) => (
      <Route
        key={path}
        path={path}
        element={isPrivate && (account ? <Page /> : <Content heading="In order to use app, please login" />)}
      />
    ));

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
