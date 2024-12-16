import { HexString } from '@polkadot/util/types';
import { TokenMetadata } from 'app/utils/sails/nft';
import { ItemState, Auction } from 'app/utils/sails/nft_marketplace';

type BaseNFT = TokenMetadata & {
  token_id: number;
  owner: HexString;
};

type NFTDetails = {
  royalty: string;
  rarity: string;
  attributes: { [key: string]: string };
};

type Offers = { [key: `[null,${string}]`]: HexString };

type MarketNFT = ItemState;

type NFT = BaseNFT & MarketNFT;

type MarketplaceState = { AllItems: MarketNFT[] };
type MarketNFTState = { ItemInfo: MarketNFT };
type NFTState = { Token: { token: BaseNFT } };
type OwnersNFTState = { TokensForOwner: { tokens: BaseNFT[] } };

type Filter = {
  value: string;
  list: string[];
  onChange: (filter: string) => void;
};

type Listing = {
  heading: string;
  description: string;
  owner: HexString;
  src: string;
  currentWinner?: HexString;
  offers?: { bidder: string; price: string }[];
  price?: MarketNFT['price'];
  rarity?: string;
  attrs?: NFTDetails['attributes'];
};

type AuctionFormValues = {
  duration: string;
  minPrice: string;
};

export type {
  BaseNFT,
  NFTDetails,
  Offers,
  Auction,
  MarketNFT,
  NFT,
  MarketplaceState,
  MarketNFTState,
  NFTState,
  OwnersNFTState,
  Filter,
  Listing,
  AuctionFormValues,
};
