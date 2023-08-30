import { Route, Routes } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { Videos } from './videos';
import { Home } from './home';
import { Subscription } from './subscription';
import { Video } from './video';

const routes = [
  { path: '/', Page: Videos },
  { path: 'subscription', Page: Subscription },
  { path: 'video/:cid', Page: Video },
];

function Routing() {
  const { account } = useAccount();
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return account ? <Routes>{getRoutes()}</Routes> : <strong>In order to use the app, please login</strong>;
}

export { Routing };
