import { atom } from 'jotai';
import { HexString } from '@polkadot/util/types';
import { MasterContractState, NFT } from './types';

const TESTNET_NFT_CONTRACT_ADDRESS = process.env.REACT_APP_TESTNET_NFT_CONTRACT_ADDRESS as HexString;

const NFT_CONTRACTS_ATOM = atom<MasterContractState['nfts'] | undefined>(undefined);
const NFTS_ATOM = atom<NFT[] | undefined>(undefined);

export { TESTNET_NFT_CONTRACT_ADDRESS, NFT_CONTRACTS_ATOM, NFTS_ATOM };
