import { HexString } from '@polkadot/util/types';

type Token = {
  owner: HexString;
  name: string;
  description: string;
  mediaUrl: string;
  attribUrl?: string | string[];
};

type NFT = Token & {
  id: string;
  collection: string;
  programId: HexString;
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

type TestnetNFTState = NFTContractState & {
  constraints: {
    authorizedMinters: HexString[];
  };
};

export type { NFT, MasterContractState, NFTContractState, TestnetNFTState };
