import { useAtom } from 'jotai';
import { useSearchParams } from 'react-router-dom';
import { useAccount, Account, useAlert } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useWallet } from '@/features/wallet/hooks';
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

function useAutoLogin() {
  const { login, wallets, isAccountReady } = useAccount();
  const alert = useAlert();

  const [searchParams, setSearchParams] = useSearchParams();

  useEffect(() => {
    if (!isAccountReady) return;

    const accountAddress = searchParams.get('account');

    if (accountAddress) {
      const account = Object.values(wallets || {})
        .flatMap(({ accounts }) => accounts)
        .find((_account) => _account?.address === accountAddress);

      if (account) {
        login(account);
        searchParams.delete('account');
        setSearchParams(searchParams);
      } else {
        alert.error(`Account with address ${accountAddress} not found`);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams, wallets, isAccountReady]);
}

export { useAuth, useAuthSync, useAutoLogin };
