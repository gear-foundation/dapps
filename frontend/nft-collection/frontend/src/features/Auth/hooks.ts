import { useLocation, useNavigate } from 'react-router';
import { useAtom } from 'jotai';
import { useAlert, useAccount, Account } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { AUTH_MESSAGE, AUTH_TOKEN_LOCAL_STORAGE_KEY } from './consts';
import { AUTH_TOKEN_ATOM, IS_AUTH_READY_ATOM, TESTNET_USERNAME_ATOM } from './atoms';
import { fetchAuth, post } from './utils';
import { AuthResponse, ISignInError, SignInResponse } from './types';
import { LOGIN, NOT_AUTHORIZED, MAIN } from '@/routes';

function useAuth() {
  const { account, login, logout } = useAccount();
  const alert = useAlert();
  const [authToken, setAuthToken] = useAtom(AUTH_TOKEN_ATOM);
  const [isAuthReady, setIsAuthReady] = useAtom(IS_AUTH_READY_ATOM);
  const [testnameUsername, setTestnameUsername] = useAtom(TESTNET_USERNAME_ATOM);

  const navigate = useNavigate();
  const location = useLocation();
  const from = location.state?.from?.pathname || MAIN;
  const isAwaitingVerification = account && !authToken;

  // eslint-disable-next-line @typescript-eslint/no-shadow
  const signIn = async (account: Account) => {
    const { address } = account;

    try {
      const { signer } = await web3FromAddress(address);
      if (!signer.signRaw) throw new Error('signRaw not exists');

      const { signature } = await signer.signRaw({
        address,
        data: AUTH_MESSAGE,
        type: 'bytes',
      });

      const res = await post('auth/login', {
        signature,
        publicKey: address,
        message: AUTH_MESSAGE,
      });

      if (!res.ok) {
        const data: ISignInError = await res.json();
        alert.error(data.message);

        setAuthToken(null);
        setTestnameUsername('');
        await login(account);
        navigate(`/${NOT_AUTHORIZED}`, { replace: true });
      } else {
        const data: SignInResponse = await res.json();
        const { accessToken, username } = data;
        await login(account);
        setAuthToken(accessToken);
        setTestnameUsername(username || '');
        navigate(from, { replace: true });
      }
    } catch (e) {
      alert.error(`${e}`);
    }
  };

  const signOut = () => {
    logout();
    setAuthToken(null);
    navigate(LOGIN, { replace: true });
  };

  const auth = () => {
    const localStorageToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    if (!localStorageToken) {
      setIsAuthReady(true);
      if (!isAwaitingVerification) logout();
      return;
    }

    fetchAuth<AuthResponse>('auth/me', 'PUT', localStorageToken)
      .then(({ username }) => {
        setAuthToken(localStorageToken);
        setTestnameUsername(username || '');
      })
      .catch(({ message }: Error) => {
        signOut();
        localStorage.removeItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);
        alert.error(message);
      })
      .finally(() => setIsAuthReady(true));
  };

  return { authToken, signIn, signOut, auth, isAuthReady, isAwaitingVerification, testnameUsername };
}

function useAuthSync() {
  const { isAccountReady } = useAccount();
  const { authToken, isAuthReady, auth } = useAuth();

  useEffect(() => {
    if (!isAccountReady) return;
    auth();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady]);

  useEffect(() => {
    if (!isAuthReady) return;
    if (!authToken) return localStorage.removeItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken);
  }, [isAuthReady, authToken]);
}

export { useAuth, useAuthSync };
