import { useAccount, useApi, useBalanceFormat, useDeriveBalancesAll } from '@gear-js/react-hooks';
import cx from 'clsx';
import { useState } from 'react';

import { ReactComponent as VaraSVG } from '../../assets/vara.svg';
import { WalletModal } from '../wallet-modal';
import { UI_CONFIG } from '../ui-config';
import styles from './wallet.module.css';

type Props = {
  variant?: 'gear' | 'vara';
};

function Wallet({ variant = 'vara' }: Props) {
  const { isApiReady } = useApi();
  const { account, isAccountReady } = useAccount();

  const { getFormattedBalance } = useBalanceFormat();
  const balances = useDeriveBalancesAll(account?.decodedAddress);
  const balance = isApiReady && balances ? getFormattedBalance(balances.freeBalance) : undefined;

  const [isModalOpen, setIsModalOpen] = useState(false);
  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);

  if (!isAccountReady) return;
  const { Button, AccountButton } = UI_CONFIG[variant];

  return (
    <>
      <div className={styles.wallet}>
        {balance && (
          <div className={styles.balance}>
            <VaraSVG />

            <p className={cx(styles.text, styles[variant])}>
              <span className={styles.value}>{balance.value}</span>
              <span className={styles.unit}>{balance.unit}</span>
            </p>
          </div>
        )}

        {account ? (
          <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
        ) : (
          <Button text="Connect Wallet" color="primary" onClick={openModal} />
        )}
      </div>

      {isModalOpen && <WalletModal variant={variant} close={closeModal} />}
    </>
  );
}

export { Wallet };
