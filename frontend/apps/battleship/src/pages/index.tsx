import { Navigate, Route, Routes, useLocation } from 'react-router-dom';
import { lazy, Suspense } from 'react';
import { ROUTES } from '@/app/consts';
import { Loader } from '@/components';
import { useAccount } from '@gear-js/react-hooks';

const routes = [
  { path: ROUTES.HOME, Page: lazy(() => import('./login')) },
  { path: ROUTES.GAME, Page: lazy(() => import('./game')), isPrivate: true },
  { path: ROUTES.NOTFOUND, Page: lazy(() => import('./not-found')) },
];

function RequireAuth({ children }: { children: JSX.Element }) {
  const { account } = useAccount();
  const location = useLocation();

  if (!account) {
    // Redirect them to the /login page, but save the current location they were
    // trying to go to when they were redirected. This allows us to send them
    // along to that page after they log in, which is a nicer user experience
    // than dropping them off on the home page.
    return <Navigate to={ROUTES.HOME} state={{ from: location }} replace />;
  }

  return children;
}

export function Routing() {
  return (
    <Routes>
      {routes.map(({ path, Page, isPrivate }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<Loader />}>
              {isPrivate ? (
                <RequireAuth>
                  <Page />
                </RequireAuth>
              ) : (
                <Page />
              )}
            </Suspense>
          }
        />
      ))}
    </Routes>
  );
}
