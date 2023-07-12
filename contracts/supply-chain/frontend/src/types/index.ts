import { HexString } from '@polkadot/util/types';

type InitPayload = {
  fungible_token: HexString;
  non_fungible_token: HexString;
  producers: HexString[];
  distributors: HexString[];
  retailers: HexString[];
};

type Item = {
  producer: HexString;
  distributor: HexString;
  retailer: HexString;
  state: { state: string; by: string };
  price: string;
  deliveryTime: string;
};

type Items = [id: string, item: Item][];

type Token = {
  id: string;
  ownerId: HexString;
  name: string;
  description: string;
  media: string;
  reference: string;
  approvedAccountIds: HexString[];
};

export type { InitPayload, Item, Items, Token };
