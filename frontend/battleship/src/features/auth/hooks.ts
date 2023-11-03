import { useAlert, useAccount } from '@gear-js/react-hooks';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useLocation, useNavigate } from 'react-router-dom';
import { ROUTES } from '@/app/consts';

export function useAuth() {
  const { login, logout } = useAccount();

  const navigate = useNavigate();
  const location = useLocation();
  const from = location.state?.from?.pathname || ROUTES.HOME;

  const alert = useAlert();

  const signIn = async (account: InjectedAccountWithMeta) => {
    const { address } = account;

    try {
      const { signer } = await web3FromAddress(address);
      if (!signer.signRaw) throw new Error('signRaw not exists');

      await login(account);
      navigate(from, { replace: true });
    } catch (e) {
      alert.error(`${e}`);
    }
  };

  const signOut = () => {
    logout();
  };

  return {
    signIn,
    signOut,
  };
}
