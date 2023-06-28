import { Route, Routes } from 'react-router-dom';
import { ROUTES } from 'consts';
import { Home } from './home';
import { Leaderboard } from './leaderboard';

const routes = [
  { path: ROUTES.HOME, Page: Home },
  { path: ROUTES.LEADERBOARD, Page: Leaderboard },
];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
