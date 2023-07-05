import { NFTs, TestnetNFT as TestnetNFTFeature } from 'features/nfts';

function TestnetNFT() {
  return (
    <>
      <TestnetNFTFeature />
      <NFTs slider />
    </>
  );
}

export { TestnetNFT };
