import { Suspense, useEffect } from 'react';
import { Route, useNavigate, useLocation } from 'react-router-dom';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { ROUTES } from '@/app/consts';
import { Loader, NotAuthorized } from '@/components';
import { useAccount } from '@gear-js/react-hooks';

import Home from './home';
import { useMyBattleQuery } from '@/app/utils';
import ImportCharacterPage from './import-character';
import GenerateCharacterPage from './generate-character';
import CreateGamePage from './create-game';
import FindGamePage from './find-game';
import NotFoundPage from './not-found';
import WaitingPage from './waiting';
import GamePage from './game';
import OnboardingPage from './onboarding';

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
