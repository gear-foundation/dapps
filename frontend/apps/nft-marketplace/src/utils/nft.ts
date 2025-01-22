import { useBalanceFormat } from '@gear-js/react-hooks';
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
  const { token_id, auction, price, media, name } = nft;
  const { getFormattedBalance } = useBalanceFormat();
  const { current_price } = auction || {};
  const isAuction = !!auction;

  const path = `/listing/${token_id}`;
  const src = getIpfsAddress(media);
  const text = `#${token_id}`;

  const actualPrice = price ?? current_price;

  const priceProp = {
    heading: isAuction ? 'Top bid' : 'Price',
    text: actualPrice ? getFormattedBalance(actualPrice).value : 'None',
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

  return offers.map(([key, bidder]) => {
    const [price] = String(key[1]).match(numberRegex) || [''];

    return { price, bidder };
  });
}

function getListingProps(baseNft: BaseNFT, marketNft: MarketNFT | null | undefined, details: NFTDetails | undefined) {
  const { name, description, media, token_id, owner } = baseNft;
  const { auction } = marketNft || {};
  const { rarity, attributes } = details || {};

  const heading = `${name} #${token_id}`;
  const src = getIpfsAddress(media);

  const price = auction ? auction.current_price : marketNft?.price;

  const currentWinner = auction?.current_winner;

  const offers = auction ? undefined : getOffers(marketNft?.offers);

  return { heading, description, owner, currentWinner, price, src, rarity, attrs: attributes, offers };
}

function getAuctionDate(auction: Auction) {
  const { started_at, ended_at } = auction;

  const formattedStartedAt = Number(started_at);
  const formattedEndedAt = Number(ended_at);

  const currentTimestamp = new Date().getTime();
  const startDate = new Date(formattedStartedAt).toLocaleString();
  const endDate = new Date(formattedEndedAt).toLocaleString();
  const isAuctionOver = currentTimestamp > formattedEndedAt;

  return { startDate, endDate, isAuctionOver };
}

export { getNFTProps, getListingProps, getAuctionDate };
