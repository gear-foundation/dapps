import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { useAccount, Account } from '@gear-js/react-hooks';
import { useWallet } from '../Wallet/hooks';
import { IS_AUTH_READY_ATOM, USER_ADDRESS_ATOM } from './atoms';
import { CB_UUID_KEY } from './consts';

export function useAuth() {
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM);
  const [userAddress, setIsUserAddress] = useAtom(USER_ADDRESS_ATOM);

  const { login, logout } = useAccount();
  const { resetWalletId } = useWallet();

  const signOut = () => {
    logout();
    resetWalletId();
    localStorage.removeItem(CB_UUID_KEY);
  };

  const auth = async () => {
    setIsAuthReady(true);
  };

  const signIn = async (_account: Account) => {
    await login(_account);
  };

  return { signIn, signOut, auth, isAuthReady, userAddress };
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
