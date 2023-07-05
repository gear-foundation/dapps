import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { LOCAL_STORAGE, SEARCH_PARAMS } from 'consts';
import { useLocation, useSearchParams } from 'react-router-dom';
import { CONTRACT_ADDRESS_ATOM } from './consts';

function useContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);

  return address;
}

function useContractAddressSetup() {
  const { pathname } = useLocation();
  const [searchParams, setSearchParams] = useSearchParams();

  const address = useContractAddress();

  useEffect(() => {
    if (!address) return;

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);

    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, address);
    setSearchParams(searchParams, { replace: true });

    // looking for pathname, cuz searchParams is not enough in case of page's <Navigate />
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address, searchParams, pathname]);
}

export { useContractAddress, useContractAddressSetup };
