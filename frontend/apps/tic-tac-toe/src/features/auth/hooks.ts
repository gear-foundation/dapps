import { useAtom } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { IS_AUTH_READY_ATOM } from './consts';
import { useWallet } from '@/features/wallet/hooks';
import { useEffect } from 'react';

export function useAuth() {
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

  const signIn = async (_account: InjectedAccountWithMeta) => {
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

export { useAuthSync };
