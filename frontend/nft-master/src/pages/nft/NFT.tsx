import { useAccount } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { NFTs, NFT as NFTFeature, useNFTs } from 'features/nfts';
import { NFT as NFTType } from 'features/nfts/types';
import { useParams } from 'react-router-dom';

type Params = {
  programId: HexString;
  id: string;
};

function NFT() {
  const { programId, id } = useParams() as Params;
  const { account } = useAccount();
  const nfts = useNFTs();

  const isCurrentNFT = (nft: NFTType) => nft.programId === programId && nft.id === id;

  const list = nfts.filter((nft) => nft.owner === account?.decodedAddress && !isCurrentNFT(nft));
  const item = nfts.find((nft) => isCurrentNFT(nft));

  return (
    <>
      {item && <NFTFeature item={item} />}
      {account && <NFTs list={list} slider />}
    </>
  );
}

export { NFT };
