import { Route } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { OnLogin, InfoText } from '@/components';

import { Create } from './create';
import { Listing } from './listing';
import { Listings } from './listings';
import { Me } from './me';

const routes = [
  { path: '/', Page: Listings },
  { path: '/listing/:id', Page: Listing },
  { path: '/create', Page: Create, isPrivate: true },
  { path: '/me', Page: Me, isPrivate: true },
];

function Routing() {
  const getRoutes = () =>
    routes.map(({ path, Page, isPrivate }) => (
      <Route
        key={path}
        path={path}
        element={
          isPrivate ? (
            <OnLogin fallback={<InfoText text="In order to use all marketplace features, please login" />}>
              <Page />
            </OnLogin>
          ) : (
            <Page />
          )
        }
      />
    ));

  return <ErrorTrackingRoutes>{getRoutes()}</ErrorTrackingRoutes>;
}

export { Routing };
