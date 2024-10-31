import { atom, useAtomValue, useSetAtom } from 'jotai';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { useAccountAvailableBalance, useAccountAvailableBalanceSync } from '@/features/account-available-balance/hooks';
import { useEffect } from 'react';
import { useAuth } from '@/features/auth';

const isAppReadyAtom = atom<boolean>(false);

export function useIsAppReady() {
  const isAppReady = useAtomValue(isAppReadyAtom);
  const setIsAppReady = useSetAtom(isAppReadyAtom);

  return { isAppReady, setIsAppReady };
}

export function useIsAppReadySync() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { isAvailableBalanceReady } = useAccountAvailableBalance();
  const { isAuthReady } = useAuth();

  const { setIsAppReady } = useIsAppReady();

  useAccountAvailableBalanceSync();
  console.log('----------------');
  console.log(isApiReady);
  console.log(isAccountReady);
  console.log(isAvailableBalanceReady);
  console.log(isAuthReady);
  useEffect(() => {
    setIsAppReady(isApiReady && isAccountReady && isAvailableBalanceReady && isAuthReady);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, isApiReady, isAvailableBalanceReady, isAuthReady]);
}
