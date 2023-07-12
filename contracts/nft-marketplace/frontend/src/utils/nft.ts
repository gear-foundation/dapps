import { withoutCommas } from '@gear-js/react-hooks';
import { ButtonProps } from '@gear-js/ui';
import { Auction, BaseNFT, MarketNFT, NFT, NFTDetails } from 'types';
import { getIpfsAddress } from 'utils';

function getButtonText(isOwner: boolean, isAuction: boolean) {
  if (!isOwner) {
    if (isAuction) return 'Make bid';

    return 'Buy now';
  }

  return 'Your item';
}

function getNFTProps(nft: NFT, isOwner: boolean) {
  const { id, auction, price, media, name } = nft;

  const { currentPrice } = auction || {};
  const isAuction = !!auction;

  const path = `/listing/${id}`;
  const src = getIpfsAddress(media);
  const text = `#${id}`;

  const priceProp = {
    heading: isAuction ? 'Top bid' : 'Price',
    text: String(price ?? currentPrice ?? 'None'),
  };

  const buttonProp = {
    text: getButtonText(isOwner, isAuction),
    color: (isOwner ? 'secondary' : 'primary') as ButtonProps['color'],
  };

  return { name, path, src, text, button: buttonProp, price: priceProp };
}

function getOffers(offers: MarketNFT['offers'] | undefined) {
  if (!offers) return [];

  const numberRegex = /\d+/;

  return Object.entries(offers).map(([key, bidder]) => {
    const [price] = key.match(numberRegex) || [''];

    return { price, bidder };
  });
}

function getListingProps(baseNft: BaseNFT, marketNft: MarketNFT | null | undefined, details: NFTDetails | undefined) {
  const { id, name, description, ownerId, media } = baseNft;
  const { auction } = marketNft || {};
  const { rarity, attributes } = details || {};

  const heading = `${name} #${id}`;
  const src = getIpfsAddress(media);

  const price = auction ? auction.currentPrice : marketNft?.price;

  const offers = auction ? undefined : getOffers(marketNft?.offers);

  return { heading, description, owner: ownerId, price, src, rarity, attrs: attributes, offers };
}

function getAuctionDate(auction: Auction) {
  const { startedAt, endedAt } = auction;

  const formattedStartedAt = +withoutCommas(startedAt);
  const formattedEndedAt = +withoutCommas(endedAt);

  const currentTimestamp = new Date().getTime();
  const startDate = new Date(formattedStartedAt).toLocaleString();
  const endDate = new Date(formattedEndedAt).toLocaleString();
  const isAuctionOver = currentTimestamp > formattedEndedAt;

  return { startDate, endDate, isAuctionOver };
}

export { getNFTProps, getListingProps, getAuctionDate };
