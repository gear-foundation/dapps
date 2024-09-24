import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';

import { AccountButton } from '../account-button';
import { WalletModal } from '../wallet-modal';
import { VaraBalance } from '../vara-balance';
import styles from './wallet.module.css';

function Wallet() {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  if (!isAccountReady) return;

  return (
    <>
      <div className={styles.wallet}>
        <VaraBalance />

        {account ? (
          <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
        ) : (
          <Button text="Connect Wallet" color="primary" onClick={openModal} />
        )}
      </div>

      {isModalOpen && <WalletModal close={closeModal} />}
    </>
  );
}

export { Wallet };
