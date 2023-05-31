import { Welcome } from 'features/welcome';
import { NFTs } from 'features/nfts';
import { useContractAddress } from 'features/contract-address';
import { useLayoutEffect } from 'react';

function Home() {
  const contractAddress = useContractAddress();

  useLayoutEffect(() => {
    if (!contractAddress) return document.body.classList.add('setup');

    document.body.classList.add('active');

    return () => {
      document.body.classList.value = '';
    };
  }, [contractAddress]);

  return (
    <>
      <Welcome />
      {contractAddress && <NFTs />}
    </>
  );
}

export { Home };
