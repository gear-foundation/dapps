import { HexString } from '@polkadot/util/types';

type Params = {
  id: string;
};

type Token = {
  approvedAccountIds: HexString[];
  description: string;
  id: string;
  media: string;
  name: string;
  ownerId: HexString;
  reference: string;
};

type Attributes = {
  [key: string]: string;
};

type TokenDetails = {
  rarity?: string;
  attributes?: Attributes;
};

export type { Params, Token, Attributes, TokenDetails };
