import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';

import { useGetMarketQuery, useOwnerOfQuery, useTokenMetadataByIdQuery } from '@/app/utils';
import { Loader } from '@/components';
import { ENV } from '@/consts';
import { useMarketplaceActions } from '@/hooks';
import { NFTDetails } from '@/types';
import { getAuctionDate, getIpfsAddress, getListingProps } from '@/utils';

import { AuctionListing } from './auction-listing';
import { OwnerListing } from './owner-listing';
import { SaleListing } from './sale-listing';

type Params = {
  id: string;
};

function Listing() {
  const { id } = useParams() as Params;
  const { account } = useAccount();

  const { tokenMetadata, isFetched: isTokenMetadataFetched } = useTokenMetadataByIdQuery({ tokenId: id });
  const { owner: nftOwner, isFetched: isOwnerFetched } = useOwnerOfQuery({ tokenId: id });
  const { market, isFetched: isMarketFetched } = useGetMarketQuery();
  const marketNft = market?.items.find(([_, { token_id }]) => Number(id) === Number(token_id))?.[1];
  const isFetched = isTokenMetadataFetched && isOwnerFetched && isMarketFetched;

  const owner = marketNft?.owner || nftOwner;
  const isOwner = account?.decodedAddress === owner;
  const isMarketOwner = nftOwner === ENV.MARKETPLACE_CONTRACT;
  const baseNft = tokenMetadata && owner ? { ...tokenMetadata, owner } : null;

  const { reference } = tokenMetadata || {};
  const { price, auction } = marketNft || {};

  const isSale = !!price;
  const isAuction = !!auction;
  const isListed = isSale || isAuction;

  const { buy, offer, bid, settle, startSale, startAuction } = useMarketplaceActions(id, price, isMarketOwner);
  const [details, setDetails] = useState<NFTDetails>();
  const isReferenceLoaded = reference ? !!details : true;

  useEffect(() => {
    if (!reference) return;

    fetch(getIpfsAddress(reference))
      .then((response) => response.json())
      .then((result) => setDetails(result));
  }, [reference]);

  return baseNft && isFetched && isReferenceLoaded ? (
    <>
      {isSale && (
        <SaleListing
          isOwner={isOwner}
          item={getListingProps(baseNft, marketNft, details)}
          onBuySubmit={buy}
          onOfferSubmit={offer}
        />
      )}

      {isAuction && (
        <AuctionListing
          isOwner={isOwner}
          item={getListingProps(baseNft, marketNft, details)}
          date={getAuctionDate(auction)}
          onBidSubmit={bid}
          onSettleSubmit={settle}
        />
      )}

      {!isListed && (
        <OwnerListing
          isOwner={isOwner}
          item={getListingProps(baseNft, marketNft, details)}
          onAuctionSubmit={startAuction}
          onSaleSubmit={startSale}
        />
      )}
    </>
  ) : (
    <Loader />
  );
}

export { Listing };
