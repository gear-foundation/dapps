import { Welcome } from 'features/welcome';
import { NFTs } from 'features/nfts';
import { useContractAddress } from 'features/contract-address';

function Home() {
  const contractAddress = useContractAddress();

  return (
    <>
      <Welcome />
      {contractAddress && <NFTs />}
    </>
  );
}

export { Home };
