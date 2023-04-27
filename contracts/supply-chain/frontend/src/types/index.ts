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
  price: number;
  deliveryTime: number;
};

type Items = [id: number, item: Item][];

type Token = {
  id: number;
  ownerId: HexString;
  name: string;
  description: string;
  media: string;
  reference: string;
  approvedAccountIds: HexString[];
};

export type { InitPayload, Item, Items, Token };
