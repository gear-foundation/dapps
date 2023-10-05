import { useAtom } from 'jotai';
import { useAccount, Account } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useWallet } from 'features/wallet/hooks';
import { IS_AUTH_READY_ATOM } from './consts';

function useAuth() {
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM);
  const { login, logout } = useAccount();
  const { resetWalletId } = useWallet();

  const signOut = () => {
    logout();
    resetWalletId();
  };

  const auth = async () => {
    setIsAuthReady(true);
  };

  const signIn = async (_account: Account) => {
    await login(_account);
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

export { useAuth, useAuthSync };
