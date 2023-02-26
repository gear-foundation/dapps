import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Battle } from './battle';
import { useInitBattleData } from 'app/hooks/use-battle';
import { Test } from './test';

const routes = [
  { path: '/', Page: Home },
  { path: '/battle', Page: Battle },
  { path: '/test', Page: Test },
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
