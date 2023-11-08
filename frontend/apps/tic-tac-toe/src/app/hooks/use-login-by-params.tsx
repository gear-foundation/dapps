import { useLocation } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useMemo } from 'react';
import { WALLET_ID_LOCAL_STORAGE_KEY } from '@/features/wallet/consts';
import { AUTH_TOKEN_LOCAL_STORAGE_KEY } from '@/features/auth/consts';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY } from '@/app/consts';

export function useLoginByParams() {
  const { search } = useLocation();
  const { login, accounts } = useAccount();

  const query = useMemo(() => new URLSearchParams(search), [search]);

  useEffect(() => {
    const isAccount = localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY);
    const isWallet = localStorage.getItem(WALLET_ID_LOCAL_STORAGE_KEY);
    const isAuthToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    if (query.size) {
      const account = query.get(ACCOUNT_ID_LOCAL_STORAGE_KEY);
      const wallet = query.get(WALLET_ID_LOCAL_STORAGE_KEY);
      const authToken = query.get(AUTH_TOKEN_LOCAL_STORAGE_KEY);

      const isEmptyStorage = !isAuthToken && !isAccount && !isWallet;
      const isSameUser = isAccount === account;
      const isDataCorrect = account && authToken && wallet;

      if ((isEmptyStorage || !isSameUser) && !!isDataCorrect) {
        localStorage.setItem(ACCOUNT_ID_LOCAL_STORAGE_KEY, account);
        localStorage.setItem(WALLET_ID_LOCAL_STORAGE_KEY, wallet);
        localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken);

        const candidate = accounts?.find((a) => a.address === account);
        if (candidate) login(candidate);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [accounts, query]);
}
