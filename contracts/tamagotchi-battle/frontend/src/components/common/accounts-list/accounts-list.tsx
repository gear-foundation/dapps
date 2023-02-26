import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useAccount } from '@gear-js/react-hooks';
import { AccountButton } from 'components/common/account-button';
import { LOCAL_STORAGE } from 'app/consts';
import { isLoggedIn } from 'app/utils';
import { useApp, useBattle } from 'app/context';
import { useNavigate } from 'react-router-dom';

type Props = {
  list: InjectedAccountWithMeta[];
  onChange: () => void;
};

export const AccountsList = ({ list, onChange }: Props) => {
  const { setIsAdmin } = useApp();
  const { battle } = useBattle();
  const { logout, login } = useAccount();
  const navigate = useNavigate();

  const onClick = async (account: InjectedAccountWithMeta) => {
    await logout();
    await login(account);
    localStorage.setItem(LOCAL_STORAGE.ACCOUNT, account.address);
    onChange();
    setIsAdmin(false);
    if (battle?.state === 'Registration') navigate('/');
  };

  return list.length > 0 ? (
    <ul className="space-y-4 w-full">
      {list.map((account) => (
        <li key={account.address}>
          <AccountButton
            address={account.address}
            name={account.meta.name}
            isActive={isLoggedIn(account)}
            onClick={() => onClick(account)}
          />
        </li>
      ))}
    </ul>
  ) : (
    <p>
      No accounts found. Please open Polkadot extension, create a new account or import existing one and reload the
      page.
    </p>
  );
};
