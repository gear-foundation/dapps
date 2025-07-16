import { useAccount } from '@gear-js/react-hooks';
import { JSX, lazy, Suspense } from 'react';
import { Navigate, Route, useLocation } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { ROUTES } from '@/app/consts';
import { useOnboarding } from '@/app/hooks';
import { Loader } from '@/components';

const routes = [
  { path: ROUTES.HOME, Page: lazy(() => import('./home')), isPrivate: true },
  { path: ROUTES.GAME, Page: lazy(() => import('./game')), isPrivate: true },
  { path: ROUTES.ONBOARDING, Page: lazy(() => import('./onboarding')) },
  { path: ROUTES.LOGIN, Page: lazy(() => import('./login')) },
  { path: ROUTES.CREATE_GAME, Page: lazy(() => import('./create-game')), isPrivate: true },
  { path: ROUTES.ROOMS, Page: lazy(() => import('./rooms')), isPrivate: true },
  { path: ROUTES.NOTFOUND, Page: lazy(() => import('./not-found')), isPrivate: true },
  { path: ROUTES.COMBINATIONS, Page: lazy(() => import('./combinations')), isPrivate: true },
];

function RequireAuth({ children }: { children: JSX.Element }) {
  const { account } = useAccount();
  const location = useLocation();

  if (!account) {
    return <Navigate to={ROUTES.LOGIN} state={{ from: location }} replace />;
  }

  return children;
}

function RequireOnboarding({ children }: { children: JSX.Element }) {
  const { account } = useAccount();
  const location = useLocation();
  const { isOnboardingPassed } = useOnboarding();

  if (!account && !isOnboardingPassed && location.pathname !== ROUTES.ONBOARDING) {
    return <Navigate to={ROUTES.ONBOARDING} />;
  }

  return children;
}

export function Routing() {
  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page, isPrivate }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<Loader />}>
              <RequireOnboarding>
                {isPrivate ? (
                  <RequireAuth>
                    <Page />
                  </RequireAuth>
                ) : (
                  <Page />
                )}
              </RequireOnboarding>
            </Suspense>
          }
        />
      ))}
    </ErrorTrackingRoutes>
  );
}
