import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { LOCAL_STORAGE } from 'consts';
import { CONTRACT_ADDRESS_ATOM } from './consts';

function useContractAddress() {
  const [address, setAddress] = useAtom(CONTRACT_ADDRESS_ATOM);

  const { isAccountReady, account } = useAccount();
  const accountAddress = account?.address;

  useEffect(() => {
    if (!address) return localStorage.removeItem(LOCAL_STORAGE.CONTRACT_ADDRESS);

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);
  }, [address]);

  useEffect(() => {
    if (isAccountReady && !accountAddress) setAddress(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, accountAddress]);

  return address;
}

export { useContractAddress };
