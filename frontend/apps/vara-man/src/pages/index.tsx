import { Route } from 'react-router-dom';
import { Suspense, lazy } from 'react';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { useInitGame } from '@/app/hooks/use-init-game';

const routes = [
  { path: '/', Page: lazy(() => import('./home')) },
  { path: '/levels', Page: lazy(() => import('./levels')) },
  { path: '/rules', Page: lazy(() => import('./rules')) },
  { path: '/game', Page: lazy(() => import('./game')) },
  { path: '/tournament/create', Page: lazy(() => import('./tournament/create')) },
  { path: '/tournament/find', Page: lazy(() => import('./tournament/find')) },
];

export const Routing = () => {
  useInitGame();

  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<>Page {Page.name} is loading...</>}>
              <Page />
            </Suspense>
          }
        />
      ))}
    </ErrorTrackingRoutes>
  );
};
