import { atom } from 'jotai';
import { ADDRESS } from 'consts';
import { getLocalStorageMasterId, getSearchParamsMasterId } from './utils';

const CONTRACT_ADDRESS_ATOM = atom(getSearchParamsMasterId() || getLocalStorageMasterId() || ADDRESS.DEFAULT_CONTRACT);

export { CONTRACT_ADDRESS_ATOM };
