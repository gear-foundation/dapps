import { Route, Routes } from 'react-router-dom';
import { Home } from './home/Home';

const routes = [
  { path: '/', Page: Home },
];

export function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => 
  <Route key={path} path={path} element={<Page />} />
  );

  return <Routes>{getRoutes()}</Routes>;
}

