import { atom } from 'jotai';
import { ADDRESS } from 'consts';
import { getSearchParamsMasterId } from './utils';

// const CONTRACT_ADDRESS_ATOM = atom(getSearchParamsMasterId() || getLocalStorageMasterId() || ADDRESS.DEFAULT_CONTRACT);
const CONTRACT_ADDRESS_ATOM = atom(getSearchParamsMasterId() || ADDRESS.DEFAULT_CONTRACT);
const TESTNET_CONTRACT_ADDRESS_ATOM = atom(getSearchParamsMasterId() || ADDRESS.DEFAULT_TESTNET_CONTRACT);

export { CONTRACT_ADDRESS_ATOM, TESTNET_CONTRACT_ADDRESS_ATOM };
