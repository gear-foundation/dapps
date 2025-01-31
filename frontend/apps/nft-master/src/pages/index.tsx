import { Route } from 'react-router-dom';
import { ErrorTrackingRoutes } from '@dapps-frontend/error-tracking';
import { NFTs } from '@/features/nfts';
import { Home } from './home';
import { NFT } from './nft';
import { NotFound } from './not-found';

const routes = [
  { path: '/', Page: Home },
  { path: '/:id', Page: NFT },
  { path: '/list', Page: NFTs },
  { path: '*', Page: NotFound },
];

export function Routing() {
  return (
    <ErrorTrackingRoutes>
      {routes.map(({ path, Page }) => (
        <Route key={path} path={path} element={<Page />} />
      ))}
    </ErrorTrackingRoutes>
  );
}
