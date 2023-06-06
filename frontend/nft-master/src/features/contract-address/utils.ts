import { HexString } from '@polkadot/util/types';
import { LOCAL_STORAGE, SEARCH_PARAMS } from 'consts';
import { isProgramIdValid } from 'utils';

const getSearchParamsMasterId = () => {
  const searchParams = new URLSearchParams(window.location.search);
  const id = searchParams.get(SEARCH_PARAMS.MASTER_CONTRACT_ID);

  if (id && isProgramIdValid(id)) return id;
};

const getLocalStorageMasterId = () => localStorage[LOCAL_STORAGE.CONTRACT_ADDRESS] as HexString | null;

export { getSearchParamsMasterId, getLocalStorageMasterId };
