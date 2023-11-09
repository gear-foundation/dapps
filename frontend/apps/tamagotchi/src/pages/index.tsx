import { lazy, Suspense } from 'react';
import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from 'error-tracking';
import { useTamagotchiInit } from '@/app/hooks/use-tamagotchi';
import { useThrottleWasmState } from '@/app/hooks/use-read-wasm-state';
import { Loader } from '@/components/loaders/loader';
import { useLessonsInit } from '@/app/hooks/use-lessons';

const routes = [{ path: '/', Page: lazy(() => import('./home')) }];

export const Routing = () => {
  useLessonsInit();
  useTamagotchiInit();
  useThrottleWasmState();

  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page }) => (
        <Route
          key={path}
          path={path}
          element={
            <Suspense fallback={<Loader />}>
              <Page />
            </Suspense>
          }
        />
      ))}
    </ErrorTrackingRoutes>
  );
};
