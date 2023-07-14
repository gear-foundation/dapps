import { Route, Routes, useLocation } from 'react-router-dom';
import { useEffect } from 'react';
import { NFTs } from 'features/nfts';
import { Home } from './home';
import { NFT } from './nft';
import { TestnetNFT } from './testnet-nft';

const routes = [
  { path: '/', Page: Home },
  { path: '/:programId/:id', Page: NFT },
  { path: '/list', Page: NFTs },
  { path: '/testnet', Page: TestnetNFT },
];

function Routing() {
  const { pathname } = useLocation();

  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
