import { Suspense } from 'react';
import Identicon from '@polkadot/react-identicon';

import { Text } from '@/components/ui/text';
import { Button } from '@/components/ui/button';

import { useAuth } from '@/features/auth';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { useWallet } from '../../hooks';

import { AvaVaraBlack } from '@/assets/images';
import { ADDRESS } from '@/app/consts';

import styles from './WalletChange.module.scss';

type Props = {
  onClose(): void;
  openConnectWallet(): void;
};

export function WalletChange({ onClose, openConnectWallet }: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const { signOut } = useAuth();

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
              <Identicon value={address} size={34} theme="polkadot" className={styles.accountIcon} />
            </Suspense>
            <Text weight="semibold">{meta.name}</Text>
          </div>
        </li>
      );
    });

  const handleChangeButtonClick = () => {
    openConnectWallet();
    onClose();
  };

  const handleLogoutButtonClick = () => {
    signOut();
    onClose();
  };

  return (
    <div>
      <div className={styles.changeAccount}>
        <div className={styles.network}>
          <AvaVaraBlack width={32} height={32} />
          <div>
            <Text weight="semibold" size="md">
              {api?.runtimeVersion.specName.toHuman()}
            </Text>
            <Text size="sm" className={styles.address}>
              {ADDRESS.NODE}
            </Text>
          </div>
        </div>

        <hr />

        <div>
          <ul className={styles.list}>{getAccounts()}</ul>
        </div>

        <div className={styles.buttons}>
          <Button onClick={handleChangeButtonClick}>Change account</Button>
          <Button variant="black" onClick={handleLogoutButtonClick}>
            Disconnect
          </Button>
        </div>
      </div>
    </div>
  );
}
