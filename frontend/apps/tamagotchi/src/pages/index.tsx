import { Route, Routes } from 'react-router-dom';
import { useTamagotchiInit } from '@/app/hooks/use-tamagotchi';
import { useThrottleWasmState } from '@/app/hooks/use-read-wasm-state';
import { lazy, Suspense } from 'react';
import { Loader } from '@/components/loaders/loader';
import { useLessonsInit } from '@/app/hooks/use-lessons';

const routes = [{ path: '/', Page: lazy(() => import('./home')) }];

export const Routing = () => {
  useLessonsInit();
  useTamagotchiInit();
  useThrottleWasmState();

  return (
    <Routes>
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
    </Routes>
  );
};
