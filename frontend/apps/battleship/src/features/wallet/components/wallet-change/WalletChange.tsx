import { Suspense } from 'react';
import Identicon from '@polkadot/react-identicon';
import { motion } from 'framer-motion';
import { Text } from '@/components/ui/text';
import { Button } from '@/components/ui/button';

import { useAccount } from '@gear-js/react-hooks';
import { useWallet } from '../../hooks';

import styles from './WalletChange.module.scss';
import { MenuOptions } from '@dapps-frontend/ui';
import { EzSignlessTransactions, EzGaslessTransactions } from '@dapps-frontend/ez-transactions';

type Props = {
  onClose(): void;
  openConnectWallet(): void;
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
    logout();
    onClose();
  };

  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }} transition={{ duration: 0.4 }}>
      <div className={styles.changeAccount}>
        <MenuOptions
          customItems={[
            { key: 'signless', option: <EzSignlessTransactions /> },
            { key: 'gasless', option: <EzGaslessTransactions /> },
          ]}
        />
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
    </motion.div>
  );
}
