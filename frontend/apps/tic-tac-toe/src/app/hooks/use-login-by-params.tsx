import { useLocation } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useMemo } from 'react';
import { WALLET_ID_LOCAL_STORAGE_KEY } from '@/features/wallet/consts';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY } from '@/app/consts';

export function useLoginByParams() {
  const { search } = useLocation();
  const { login, wallets } = useAccount();

  const query = useMemo(() => new URLSearchParams(search), [search]);

  useEffect(() => {
    const isAccount = localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY);
    const isWallet = localStorage.getItem(WALLET_ID_LOCAL_STORAGE_KEY);

    if (query.size) {
      const account = query.get(ACCOUNT_ID_LOCAL_STORAGE_KEY);
      const wallet = query.get(WALLET_ID_LOCAL_STORAGE_KEY);

      const isEmptyStorage = !isAccount && !isWallet;
      const isSameUser = isAccount === account;
      const isDataCorrect = account && wallet;

      if ((isEmptyStorage || !isSameUser) && !!isDataCorrect) {
        localStorage.setItem(ACCOUNT_ID_LOCAL_STORAGE_KEY, account);
        localStorage.setItem(WALLET_ID_LOCAL_STORAGE_KEY, wallet);

        const candidate = Object.values(wallets || {})
          .flatMap(({ accounts }) => accounts)
          .find((acc) => acc?.address === account);

        if (candidate) login(candidate);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wallets, query]);
}
