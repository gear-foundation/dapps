import { useAccount } from '@gear-js/react-hooks';
import { NFTs, NFT as NFTFeature } from 'features/nfts';

function NFT() {
  const { account } = useAccount();

  return (
    <>
      <NFTFeature />
      {account && <NFTs slider />}
    </>
  );
}

export { NFT };
