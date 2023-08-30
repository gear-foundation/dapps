import { useAlert, useApi } from '@gear-js/react-hooks';
import { ADDRESS } from 'consts';
import { useEffect, useState } from 'react';
import { NFT, MarketNFT, BaseNFT } from 'types';
import { useMarketplace, useMarketplaceStateBuffer } from './marketplace';
import { useStateMetadata } from './metadata';
import { useNftStateBuffer, useOwnersNft } from './nft';

function useMergedNFTs() {
  const { api } = useApi();
  const alert = useAlert();

  const { NFTs: marketNFTs } = useMarketplace();
  const nftStateBuffer = useNftStateBuffer();
  const nftStateMetadata = useStateMetadata(nftStateBuffer);

  const [NFTs, setNFTs] = useState<NFT[]>([]);
  const [isEachNFTRead, setIsEachNFTRead] = useState(false);

  useEffect(() => {
    if (!marketNFTs || !nftStateBuffer || !nftStateMetadata) return;

    const combinedNFTs = marketNFTs.map((marketNft) =>
      api.programState
        .readUsingWasm(
          {
            programId: ADDRESS.NFT_CONTRACT,
            fn_name: 'token',
            wasm: nftStateBuffer,
            argument: marketNft.tokenId,
          },
          nftStateMetadata,
        )
        .then((state) => state.toHuman() as BaseNFT)
        .then((baseNft) => ({ ...marketNft, ...baseNft })),
    );

    Promise.all(combinedNFTs)
      .then((result) => {
        setNFTs(result);
        setIsEachNFTRead(true);
      })
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [marketNFTs]);

  return { NFTs, isEachNFTRead };
}

function useMergedOwnerNFTs() {
  const { api } = useApi();
  const alert = useAlert();

  const { NFTs: ownerNFTs } = useOwnersNft();
  const marketplaceStateBuffer = useMarketplaceStateBuffer();
  const marketplaceStateMetadata = useStateMetadata(marketplaceStateBuffer);

  const [NFTs, setNFTs] = useState<NFT[]>([]);
  const [isEachNFTRead, setIsEachNFTRead] = useState(false);

  useEffect(() => {
    if (!ownerNFTs || !marketplaceStateBuffer || !marketplaceStateMetadata) return;

    const combinedNFTs = ownerNFTs.map(
      (baseNft) =>
        api.programState
          .readUsingWasm(
            {
              programId: ADDRESS.MARKETPLACE_CONTRACT,
              fn_name: 'item_info',
              wasm: marketplaceStateBuffer,
              argument: { nft_contract_id: ADDRESS.NFT_CONTRACT, token_id: baseNft.id },
            },
            marketplaceStateMetadata,
          )
          .then((state) => state.toHuman() as MarketNFT)
          .then((marketNft) => ({ ...marketNft, ...baseNft })), // order is important
    );

    Promise.all(combinedNFTs)
      .then((result) => {
        setNFTs(result);
        setIsEachNFTRead(true);
      })
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [ownerNFTs, marketplaceStateBuffer, marketplaceStateMetadata]);

  return { NFTs, isEachNFTRead };
}

export { useMergedNFTs, useMergedOwnerNFTs };
