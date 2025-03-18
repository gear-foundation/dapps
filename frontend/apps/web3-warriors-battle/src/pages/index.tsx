import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { Route, useNavigate, useLocation, Navigate, Outlet } from 'react-router-dom';

import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';

import { ROUTES } from '@/app/consts';
import { useMyBattleQuery } from '@/app/utils';

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
  { path: ROUTES.ONBOARDING, Page: Onboarding },
];

const privateRoutes = [
  { path: ROUTES.IMPORT_CHARACTER, Page: ImportCharacter },
  { path: ROUTES.GENERATE_CHARACTER, Page: GenerateCharacter },
  { path: ROUTES.CREATE_GAME, Page: CreateGame },
  { path: ROUTES.FIND_GAME, Page: FindGame },
  { path: ROUTES.WAITING, Page: Waiting },
  { path: ROUTES.GAME, Page: Game },
];

function PrivateRoute() {
  const { account } = useAccount();

  return account ? <Outlet /> : <Navigate to={ROUTES.HOME} />;
}

export function Routing() {
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

  const renderRoutes = (items: typeof routes) =>
    items.map(({ path, Page }) => {
      return <Route key={path} path={path} element={<Page />} />;
    });

  return (
    <ErrorTrackingRoutes>
      {renderRoutes(routes)}
      <Route element={<PrivateRoute />}>{renderRoutes(privateRoutes)}</Route>
    </ErrorTrackingRoutes>
  );
}
