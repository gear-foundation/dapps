import { useAccount } from '@gear-js/react-hooks';
import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from 'error-tracking';
import { Content } from 'components';
import { SUBHEADING } from 'consts';
import { Home } from './home';

const routes = [{ path: '/', Page: Home }];

function Routing() {
  const { account } = useAccount();

  const getRoutes = () =>
    routes.map(({ path, Page }) => (
      <Route key={path} path={path} element={account ? <Page /> : <Content subheading={SUBHEADING.LOGIN} />} />
    ));

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
