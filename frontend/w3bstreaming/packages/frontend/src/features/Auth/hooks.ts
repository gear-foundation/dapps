import { useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useAtom } from 'jotai';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { Account, useAccount } from '@gear-js/react-hooks';
import { useWallet } from '../Wallet/hooks';
import { IS_AUTH_READY_ATOM, USER_ADDRESS_ATOM } from './atoms';
import { fetchAuth } from './utils';
import { CB_UUID_KEY } from './consts';
import { AuthResponse } from './types';

export function useAuth() {
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM);
  const [userAddress, setIsUserAddress] = useAtom(USER_ADDRESS_ATOM);
  const [query, setQuery] = useSearchParams();

  const { login, logout, account } = useAccount();
  const { resetWalletId } = useWallet();

  const resetSearchQuery = () => {
    query.delete('uuid');

    setQuery(query);
  };

  const signOut = () => {
    logout();
    resetWalletId();
    localStorage.removeItem(CB_UUID_KEY);
  };

  const auth = async () => {
    const uuid = query.get('uuid');
    const cbUuid = localStorage.getItem(CB_UUID_KEY);

    if (query.size && uuid) {
      localStorage.setItem(CB_UUID_KEY, uuid);
    }
    setIsAuthReady(false);
    if (account) {
      try {
        const res = await fetchAuth<AuthResponse>('api/user/auth', 'POST', {
          coinbaseUID: uuid || cbUuid,
          substrate: account.decodedAddress,
        });

        if (res?.success) {
          setIsUserAddress(res.content.user.address);
        }

        if (!res?.success) {
          setIsUserAddress(null);
        }

        resetSearchQuery();
      } catch (err) {
        console.log(err);
      }
    }
    setIsAuthReady(true);
  };

  const signIn = async (_account: InjectedAccountWithMeta) => {
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
