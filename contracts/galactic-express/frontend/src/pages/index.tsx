import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Battle } from './battle';
import { useInitLouncheData } from 'app/hooks/use-battle';

const routes = [
  { path: '/', Page: Home },
  { path: '/battle', Page: Battle },
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
