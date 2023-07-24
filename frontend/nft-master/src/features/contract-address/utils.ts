import { SEARCH_PARAMS } from 'consts';
import { isProgramIdValid } from 'utils';

const getSearchParamsMasterId = () => {
  const searchParams = new URLSearchParams(window.location.search);
  const id = searchParams.get(SEARCH_PARAMS.MASTER_CONTRACT_ID);

  if (id && isProgramIdValid(id)) return id;
};

export { getSearchParamsMasterId };
