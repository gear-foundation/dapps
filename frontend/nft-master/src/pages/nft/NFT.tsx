import { NFTs } from 'features/nfts';
import { NFT as NFTFeature } from 'features/nft';
import styles from './NFT.module.scss';

function NFT() {
  return (
    <>
      <NFTFeature />
      <NFTs slider />
    </>
  );
}

export { NFT };
