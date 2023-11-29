import { Button } from '@gear-js/ui';
import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { useState } from 'react';

import { ReactComponent as VaraSVG } from '../../assets/vara.svg';
import { useFreeAccountBalance } from '../../hooks';
import { AccountButton } from '../account-button';
import { WalletModal } from '../wallet-modal';
import styles from './wallet.module.css';

function Wallet() {
  const { isApiReady } = useApi();
  const { account, isAccountReady } = useAccount();

  const { getFormattedBalance } = useBalanceFormat();
  const { freeAccountBalance } = useFreeAccountBalance();
  const balance = isApiReady && freeAccountBalance ? getFormattedBalance(freeAccountBalance) : undefined;

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  return isAccountReady ? (
    <>
      <div className={styles.wallet}>
        {balance && (
          <div className={styles.balance}>
            <VaraSVG />

            <p className={styles.text}>
              <span className={styles.value}>{balance.value}</span>
              <span className={styles.unit}>{balance.unit}</span>
            </p>
          </div>
        )}

        {account ? (
          <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
        ) : (
          <Button text="Connect Wallet" color="lightGreen" onClick={openModal} />
        )}
      </div>

      {isModalOpen && <WalletModal close={closeModal} />}
    </>
  ) : null;
}

export { Wallet };
