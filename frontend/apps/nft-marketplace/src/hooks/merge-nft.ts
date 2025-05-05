import { useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { useEffect, useMemo, useState } from 'react';

import { getErrorMessage } from '@dapps-frontend/ui';

import { useGetMarketQuery, useNftProgram, useTokensForOwnerQuery } from '@/app/utils';
import { MarketState } from '@/app/utils/sails/nft_marketplace';
import { MarketNFT, NFT } from '@/types';

function useMergedNFTs(items?: MarketState['items']) {
  const { api } = useApi();
  const alert = useAlert();

  const nftProgram = useNftProgram();

  const [NFTs, setNFTs] = useState<NFT[]>([]);
  const [isEachNFTRead, setIsEachNFTRead] = useState(false);

  useEffect(() => {
    if (!api || !items || !nftProgram) return;

    const combinedNFTs = items.map(([_, marketNft]) =>
      nftProgram.vnft
        .tokenMetadataById(marketNft.token_id)
        .then((baseNft) => ({ ...marketNft, ...baseNft!, token_id: parseInt(String(marketNft.token_id), 16) })),
    );

    Promise.all(combinedNFTs)
      .then((result) => {
        setNFTs(result);
        setIsEachNFTRead(true);
      })
      .catch((error) => alert.error(getErrorMessage(error)));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, items, nftProgram]);

  return { NFTs, isEachNFTRead };
}

function useMergedAllNFTs() {
  const { market } = useGetMarketQuery();

  return useMergedNFTs(market?.items);
}

function useMergedOwnerNFTs() {
  const { ownerTokens, isFetched: isOwnerTokensFetched } = useTokensForOwnerQuery();
  const { market, isFetched: isMarketFetched } = useGetMarketQuery();
  const { account } = useAccount();

  const ownerMarketTokens = useMemo(
    () => market?.items.filter(([_, { owner }]) => owner === account?.decodedAddress),
    [market?.items],
  );

  const { NFTs: ownerMarketNFTs, isEachNFTRead: isMarketNFTRead } = useMergedNFTs(ownerMarketTokens);

  const isEachNFTRead = isOwnerTokensFetched && isMarketFetched && isMarketNFTRead;

  const ownerNFTs = useMemo(() => {
    if (!ownerTokens || !market || !account || !isEachNFTRead) return [];

    const mergedOwnerTokens = ownerTokens.map(([tokenId, baseNft]) => {
      const marketNft = market.items.find(([_, { token_id }]) => token_id === tokenId)?.[1] as MarketNFT;
      return { ...marketNft, ...baseNft, token_id: parseInt(String(tokenId), 16), owner: account.decodedAddress }; // order is important
    });

    return [...ownerMarketNFTs, ...mergedOwnerTokens];

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [ownerTokens, market, ownerMarketNFTs]);

  return { ownerNFTs, isEachNFTRead };
}

export { useMergedAllNFTs, useMergedOwnerNFTs };
