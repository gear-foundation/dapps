import { Account, useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect } from 'react';

import { useWallet } from '../Wallet/hooks';

import { IS_AUTH_READY_ATOM } from './atoms';

export function useAuth() {
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM);

  const { login, logout } = useAccount();
  const { resetWalletId } = useWallet();

  const signOut = () => {
    logout();
    resetWalletId();
  };

  const auth = () => {
    setIsAuthReady(true);
  };

  const signIn = (_account: Account) => {
    login(_account);
  };

  return { signIn, signOut, auth, isAuthReady };
}

function useAuthSync() {
  const { isAccountReady, account } = useAccount();
  const { auth } = useAuth();

  useEffect(() => {
    if (!isAccountReady) {
      return;
    }

    auth();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, account?.decodedAddress]);
}

export { useAuthSync };
