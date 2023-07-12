import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Battle } from './battle';
import { useInitBattleData } from 'features/battle/hooks';
// import { Test } from './test';
import { ROUTES } from '../app/consts';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.GAME, Page: Battle },
  // { path: ROUTES.TEST, Page: Test },
  { path: ROUTES.NOTFOUND, Page: Home },
];

export const Routing = () => {
  useInitBattleData();

  return (
    <Routes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </Routes>
  );
};
