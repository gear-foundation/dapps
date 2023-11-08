import { Route, Routes } from 'react-router-dom';
import { Suspense, lazy } from 'react';
import { useInitGame } from '@/app/hooks/use-game';

const routes = [
  { path: '/', Page: lazy(() => import('./home')) },
  { path: '/levels', Page: lazy(() => import('./levels')) },
  { path: '/rules', Page: lazy(() => import('./rules')) },
  { path: '/game', Page: lazy(() => import('./game')) },
];

export const Routing = () => {
  useInitGame();

  return (
    <Routes>
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
    </Routes>
  );
};
