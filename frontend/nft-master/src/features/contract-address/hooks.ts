import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { LOCAL_STORAGE } from 'consts';
import { CONTRACT_ADDRESS_ATOM } from './consts';

function useContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);

  return address;
}

function useContractAddressLocalStorage() {
  const address = useContractAddress();

  useEffect(() => {
    if (!address) return localStorage.removeItem(LOCAL_STORAGE.CONTRACT_ADDRESS);

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);
  }, [address]);
}

export { useContractAddress, useContractAddressLocalStorage };
