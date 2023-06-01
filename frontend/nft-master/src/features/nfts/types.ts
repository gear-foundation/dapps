import { HexString } from '@polkadot/util/types';

type Token = {
  owner: HexString;
  name: string;
  description: string;
  mediaUrl: string;
  attribUrl: string;
};

type Collection = {
  name: string;
  description: string;
};

type MasterContractState = {
  nfts: [HexString, string][];
  operators: HexString[];
};

type NFTContractState = {
  tokens: [string, Token][];
  collection: Collection;
};

export type { MasterContractState, NFTContractState };
