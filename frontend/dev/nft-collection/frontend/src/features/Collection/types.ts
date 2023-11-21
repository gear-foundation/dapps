import { HexString } from '@gear-js/api';
import { ReactElement } from 'react';

export type Token = {
  owner: string;
  medium: string;
  timeMinted: string;
  name: string;
  description: string;
  collectionName: string;
  id: string;
};

export interface Collection {
  id: string;
  timeCreation: string;
  tokens: Token[];
  owner: HexString;
  transactions: string[];
  owners: HexString[];
  collection: {
    name: string;
    description: string;
  };
  constraints: {
    minters: HexString[];
  };
  availableMedia: string[];
  nft: null | string;
  sft: null;
}

export type Collections = {
  [key: string]: Collection;
};

export interface OwnerData {
  collections: { id: string; component: ReactElement }[];
  nfts: { id: string; component: ReactElement }[];
}

export interface AllData {
  collections: { id: string; component: ReactElement }[];
  nfts: { id: string; component: ReactElement }[];
}
