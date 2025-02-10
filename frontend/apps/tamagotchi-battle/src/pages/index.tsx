import { Route } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { useInitBattleData } from '@/features/battle/hooks';

import { ROUTES } from '../app/consts';

import { Battle } from './battle';
import { Home } from './home';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.GAME, Page: Battle },
  { path: ROUTES.NOTFOUND, Page: Home },
];

export const Routing = () => {
  useInitBattleData();

  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </ErrorTrackingRoutes>
  );
};
