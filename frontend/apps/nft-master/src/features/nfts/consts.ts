import { atom } from 'jotai';
import type { RequestPolicy } from 'urql';

import { NFT } from './types';

const NFTS_ATOM = atom<NFT[] | null>(null);

const IS_MINTING_ATOM = atom<boolean>(false);

const IS_FETCHING_NFT_ATOM = atom<boolean>(false);

type UserNftQueryExecute = (options?: { requestPolicy?: RequestPolicy }) => void;

const USER_NFT_QUERY_ATOM = atom<{
  fn: UserNftQueryExecute | null;
}>({
  fn: null,
});

export { NFTS_ATOM, IS_MINTING_ATOM, IS_FETCHING_NFT_ATOM, USER_NFT_QUERY_ATOM };
