import { Route, Routes, useNavigate } from 'react-router-dom';
import { useEffect } from 'react';
import { useContractAddress } from 'features/contract-address';
import { Home } from './home';
import { NFT } from './nft';

const routes = [
  { path: '/', Page: Home },
  { path: '/nft/:id', Page: NFT },
];

function Routing() {
  const navigate = useNavigate();
  const contractAddress = useContractAddress();

  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  useEffect(() => {
    if (!contractAddress) navigate('/');
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [contractAddress]);

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
