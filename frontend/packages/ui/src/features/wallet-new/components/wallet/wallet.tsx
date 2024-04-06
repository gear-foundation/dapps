import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { AccountButton } from '../account-button';
import { WalletModal } from '../wallet-modal';
import styles from './wallet.module.css';
import { VaraBalance } from '../vara-balance';

export type ClassNameProps = {
  balance?: string;
};
type Props = {
  isWalletModalOpen?: boolean;
  walletModalHandler?: (bool: boolean) => void;
  className?: ClassNameProps;
};

function Wallet({ isWalletModalOpen, walletModalHandler, className }: Props) {
  const { account, isAccountReady } = useAccount();

  const [isModalOpen, setIsModalOpen] = useState(isWalletModalOpen || false);
  const openModal = () => walletModalHandler?.(true) || setIsModalOpen(true);
  const closeModal = () => walletModalHandler?.(false) || setIsModalOpen(false);

  useEffect(() => {
    if (isWalletModalOpen !== undefined) {
      setIsModalOpen(isWalletModalOpen);
    }
  }, [isWalletModalOpen]);

  return isAccountReady ? (
    <>
      <div className={styles.wallet}>
        <VaraBalance className={className?.balance} />

        {account ? (
          <div className={styles.accountButton}>
            <AccountButton address={account.address} name={account.meta.name} onClick={openModal} />
          </div>
        ) : (
          <Button text="Connect Wallet" color="primary" className={styles.connectButton} onClick={openModal} />
        )}
      </div>

      <WalletModal onClose={closeModal} open={isModalOpen} setOpen={openModal} />
    </>
  ) : null;
}

export { Wallet };
