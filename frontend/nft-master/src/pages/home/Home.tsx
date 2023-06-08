import { useAccount } from '@gear-js/react-hooks';
import { useLayoutEffect } from 'react';
import { Welcome } from 'features/welcome';
import { NFTs, useNFTs } from 'features/nfts';

function Home() {
  const { account } = useAccount();
  const accountAddress = account?.decodedAddress;

  const list = useNFTs();

  useLayoutEffect(() => {
    if (accountAddress) return document.body.classList.remove('welcome');

    document.body.classList.add('welcome');
  }, [accountAddress]);

  return accountAddress ? <NFTs list={list.filter(({ owner }) => owner === accountAddress)} /> : <Welcome />;
}

export { Home };
