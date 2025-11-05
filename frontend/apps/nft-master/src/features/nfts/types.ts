import { HexString } from '@polkadot/util/types';

type Token = {
  owner: { id: HexString };
  name: string;
  description: string;
  mediaUrl: string;
  attribUrl?: string | string[];
  id: HexString;
};

type NFT = Token & {
  collection: string;
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

type AccountNftsQueryResult<T = NFT> = { nfts: T[] | null };
type AccountNftsQueryVariables = { account_id: string };

type NftByIdQueryResult = { nfts: NFT[] | null };
type NftByIdQueryVariables = { id: string };

type NftsByNameQueryResult = { nfts: NFT[] | null };
type NftsByNameQueryVariables = { search_query: string | null };

export type {
  NFT,
  MasterContractState,
  NFTContractState,
  TestnetNFTState,
  AccountNftsQueryResult,
  AccountNftsQueryVariables,
  NftByIdQueryResult,
  NftByIdQueryVariables,
  NftsByNameQueryResult,
  NftsByNameQueryVariables,
};

export type IAdminsRequest = {
  Admins: HexString[];
};

export type IStorageIdByAddressRequest = {
  StorageIdByAddress: HexString;
};

export type IUserNFTRequest = NFT;
