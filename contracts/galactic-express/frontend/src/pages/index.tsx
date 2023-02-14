import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Launch } from './launch';
import { useInitLouncheData } from 'app/hooks/use-battle';
import { LeaderBoard } from "./leader-board";

const routes = [
  { path: '/', Page: Home },
  { path: '/launch', Page: Launch },
  { path: '/leader-board', Page: LeaderBoard },
];

export const Routing = () => {
  useInitLouncheData();

  return (
    <Routes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </Routes>
  );
};
