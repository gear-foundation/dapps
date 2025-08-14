import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Identicon } from '@polkadot/react-identicon';
import { EzSignlessTransactions, EzGaslessTransactions } from 'gear-ez-transactions';
import { Suspense } from 'react';

import { MenuOptions } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

import { useWallet } from '../../hooks';

import styles from './wallet-change.module.scss';

type Props = {
  onClose: () => void;
  openConnectWallet: () => void;
};

export function WalletChange({ onClose, openConnectWallet }: Props) {
  const { account, logout } = useAccount();
  const { walletAccounts } = useWallet();

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;
      const isActive = address === account?.address;
      if (!isActive) return null;

      return (
        <li key={address}>
          <div className={styles.account}>
            <Suspense>
              <Identicon value={address} size={30} theme="polkadot" className={styles.accountIcon} />
            </Suspense>
            <span>{meta.name}</span>
          </div>
        </li>
      );
    });

  const handleChangeButtonClick = () => {
    openConnectWallet();
    onClose();
  };

  const handleLogoutButtonClick = () => {
    logout();
    onClose();
  };

  return (
    <div>
      <div className={styles.changeAccount}>
        <MenuOptions
          customItems={[
            { key: 'signless', option: <EzSignlessTransactions allowedActions={SIGNLESS_ALLOWED_ACTIONS} /> },
            { key: 'gasless', option: <EzGaslessTransactions /> },
          ]}
          showDisconnect={false}
        />
        <div>
          <ul className={styles.list}>{getAccounts()}</ul>
        </div>

        <div className={styles.buttons}>
          <Button onClick={handleChangeButtonClick}>Change account</Button>
          <Button color="contrast" onClick={handleLogoutButtonClick}>
            Disconnect
          </Button>
        </div>
      </div>
    </div>
  );
}
