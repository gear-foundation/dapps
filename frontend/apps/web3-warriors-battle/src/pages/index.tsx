import { useAccount } from '@gear-js/react-hooks';
import { Suspense, useEffect } from 'react';
import { Route, useNavigate, useLocation } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { ROUTES } from '@/app/consts';
import { useMyBattleQuery } from '@/app/utils';
import { Loader, NotAuthorized } from '@/components';

import { CreateGame } from './create-game';
import { FindGame } from './find-game';
import { Game } from './game';
import { GenerateCharacter } from './generate-character';
import { Home } from './home';
import { ImportCharacter } from './import-character';
import { NotFound } from './not-found';
import { Onboarding } from './onboarding';
import { Waiting } from './waiting';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.NOTFOUND, Page: NotFound },
  { path: ROUTES.IMPORT_CHARACTER, Page: ImportCharacter },
  { path: ROUTES.GENERATE_CHARACTER, Page: GenerateCharacter },
  { path: ROUTES.CREATE_GAME, Page: CreateGame },
  { path: ROUTES.FIND_GAME, Page: FindGame },
  { path: ROUTES.WAITING, Page: Waiting },
  { path: ROUTES.GAME, Page: Game },
  { path: ROUTES.ONBOARDING, Page: Onboarding },
];

export function Routing() {
  const { account } = useAccount();

  const navigate = useNavigate();
  const location = useLocation();

  const { battleState, isFetching } = useMyBattleQuery();

  useEffect(() => {
    if (battleState && !isFetching) {
      const { state } = battleState;
      if ('registration' in state && location.pathname !== ROUTES.WAITING && location.pathname !== ROUTES.ONBOARDING) {
        navigate(ROUTES.WAITING);
      }
      if ('started' in state && location.pathname !== ROUTES.GAME && location.pathname !== ROUTES.ONBOARDING) {
        navigate(ROUTES.GAME);
      }
      if ('gameIsOver' in state && location.pathname !== ROUTES.GAME && location.pathname !== ROUTES.ONBOARDING) {
        navigate(ROUTES.GAME);
      }
    }
  }, [battleState, isFetching, navigate, location.pathname]);

  return account ? (
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
  ) : (
    <NotAuthorized />
  );
}
