import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Battle } from './battle';
import { useInitGame } from 'app/hooks/use-game';

const routes = [
  { path: '/', Page: Home },
  { path: '/battle', Page: Battle },
];

export const Routing = () => {
  useInitGame();

  return (
    <Routes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </Routes>
  );
};
