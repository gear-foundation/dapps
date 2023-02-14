import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Launch } from './launch';
import { useInitBattleData } from 'app/hooks/use-battle';

const routes = [
  { path: '/', Page: Home },
  { path: '/launch', Page: Launch },
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
