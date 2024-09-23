import { useEffect, useState, useCallback, useMemo } from 'react';
import { useLocation } from 'react-router-dom';
import { StateMetadata, getStateMetadata } from '@gear-js/api';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY } from '@/consts';
import { AUTH_TOKEN_LOCAL_STORAGE_KEY } from './features/Auth/consts';

export function useStateMetadata(source: string) {
  const alert = useAlert();

  const [stateMetadata, setStateMetadata] = useState<StateMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((buffer) => getStateMetadata(buffer))
      .then((result) => setStateMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return stateMetadata;
}

export function useMediaQuery(width: number) {
  const [targetReached, setTargetReached] = useState(false);

  const updateTarget = useCallback((e: MediaQueryListEvent) => {
    if (e.matches) {
      setTargetReached(true);
    } else {
      setTargetReached(false);
    }
  }, []);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const media = window.matchMedia(`(max-width:${width}px)`);
      media.addEventListener('change', updateTarget);

      if (media.matches) {
        setTargetReached(true);
      }

      return () => media.removeEventListener('change', updateTarget);
    }
  }, [updateTarget, width]);

  return targetReached;
}

export function useLoginByParams() {
  const { search } = useLocation();
  const { login, isAccountReady, wallets } = useAccount();

  const query = useMemo(() => new URLSearchParams(search), [search]);

  useEffect(() => {
    // TODO: auth is not needed, remove!
    const isAccount = localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY);
    const isAuthToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    if (isAccountReady && query.size) {
      const account = query.get(ACCOUNT_ID_LOCAL_STORAGE_KEY);
      const authToken = query.get(AUTH_TOKEN_LOCAL_STORAGE_KEY);

      const isEmptyStorage = !isAuthToken && !isAccount;
      const isSameUser = isAccount === account;
      const isDataCorrect = account && authToken;

      if ((isEmptyStorage || !isSameUser) && !!isDataCorrect) {
        localStorage.setItem(ACCOUNT_ID_LOCAL_STORAGE_KEY, account);
        localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken);

        const candidate = Object.values(wallets || {})
          .flatMap(({ accounts }) => accounts)
          .find((acc) => acc?.address === account);

        if (candidate) login(candidate);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, wallets, query]);
}
