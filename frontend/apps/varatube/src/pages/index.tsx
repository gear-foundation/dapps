import { useAccount } from '@gear-js/react-hooks';
import { Route } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { Subscription } from './subscription';
import { Video } from './video';
import { Videos } from './videos';

const routes = [
  { path: '/', Page: Videos, isPrivate: true },
  { path: 'subscription', Page: Subscription, isPrivate: true },
  { path: 'video/:cid', Page: Video, isPrivate: true },
];

function Routing() {
  const { account } = useAccount();

  const getRoutes = () =>
    routes.map(({ path, Page, isPrivate }) => (
      <Route
        key={path}
        path={path}
        element={isPrivate && (account ? <Page /> : <strong>In order to use the app, please login</strong>)}
      />
    ));

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
