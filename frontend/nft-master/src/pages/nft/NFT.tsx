import { NFTs, NFT as NFTFeature } from 'features/nfts';

function NFT() {
  return (
    <>
      <NFTFeature />
      <NFTs slider />
    </>
  );
}

export { NFT };
