import { atom } from 'jotai';
import { NFT } from './types';

const NFTS_ATOM = atom<NFT[] | undefined>(undefined);

export { NFTS_ATOM };
