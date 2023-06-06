import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { LOCAL_STORAGE, SEARCH_PARAMS } from 'consts';
import { useSearchParams } from 'react-router-dom';
import { CONTRACT_ADDRESS_ATOM } from './consts';

function useContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);

  return address;
}

function useContractAddressSetup() {
  const [searchParams, setSearchParams] = useSearchParams();

  const address = useContractAddress();

  useEffect(() => {
    if (!address) return localStorage.removeItem(LOCAL_STORAGE.CONTRACT_ADDRESS);

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);

    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, address);
    setSearchParams(searchParams);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [address, searchParams]);
}

export { useContractAddress, useContractAddressSetup };
