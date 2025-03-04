import { useAccount } from '@gear-js/react-hooks';
import { Suspense, useEffect } from 'react';
import { Route, useNavigate, useLocation } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { ROUTES } from '@/app/consts';
import { useMyBattleQuery } from '@/app/utils';
import { Loader, NotAuthorized } from '@/components';

import CreateGamePage from './create-game';
import FindGamePage from './find-game';
import GamePage from './game';
import GenerateCharacterPage from './generate-character';
import Home from './home';
import ImportCharacterPage from './import-character';
import NotFoundPage from './not-found';
import OnboardingPage from './onboarding';
import WaitingPage from './waiting';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.NOTFOUND, Page: NotFoundPage },
  { path: ROUTES.IMPORT_CHARACTER, Page: ImportCharacterPage },
  { path: ROUTES.GENERATE_CHARACTER, Page: GenerateCharacterPage },
  { path: ROUTES.CREATE_GAME, Page: CreateGamePage },
  { path: ROUTES.FIND_GAME, Page: FindGamePage },
  { path: ROUTES.WAITING, Page: WaitingPage },
  { path: ROUTES.GAME, Page: GamePage },
  { path: ROUTES.ONBOARDING, Page: OnboardingPage },
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
