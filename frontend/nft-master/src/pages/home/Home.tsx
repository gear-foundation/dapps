import { useAccount } from '@gear-js/react-hooks';
import { useLayoutEffect } from 'react';
import { Welcome } from 'features/welcome';
import { NFTs } from 'features/nfts';

function Home() {
  const { account } = useAccount();
  const accountAddress = account?.decodedAddress;

  useLayoutEffect(() => {
    if (accountAddress) return document.body.classList.remove('welcome');

    document.body.classList.add('welcome');
  }, [accountAddress]);

  return accountAddress ? <NFTs /> : <Welcome />;
}

export { Home };
