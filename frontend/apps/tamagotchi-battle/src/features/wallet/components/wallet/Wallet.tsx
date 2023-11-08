import { useState } from 'react';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Button, VaraBalance } from 'components/ui';
import clsx from 'clsx';
import { useAccountAvailableBalance } from 'features/account-available-balance/hooks';
import { WalletModal } from '../wallet-modal';
import styles from './Wallet.module.scss';
import { WalletIcon } from '../wallet-icon';

type WalletProps = {
  className?: string;
  isModalOpen?: boolean;
  openModal?: () => void;
  closeModal?: () => void;
};

export function Wallet({ className, isModalOpen, openModal, closeModal }: WalletProps) {
  const [open, setOpen] = useState(false);
  const { account } = useAccount();
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { availableBalance: balance } = useAccountAvailableBalance();

  const isSigned = isApiReady && isAccountReady && !!account;

  return (
    <>
      {isSigned && <VaraBalance value={balance?.value || '0'} unit={balance?.unit} className={styles.balance} />}
      <Button
        variant={isSigned ? 'black' : 'primary'}
        className={clsx(styles.button, className)}
        onClick={openModal || (() => setOpen(true))}
        disabled={!isApiReady || !isAccountReady}>
        {isSigned && <WalletIcon address={account.address} size={16} className={styles.icon} />}
        <span>{isSigned ? account.meta.name : 'Connect Wallet'}</span>
      </Button>

      {(isModalOpen || open) && <WalletModal onClose={closeModal || (() => setOpen(false))} />}
    </>
  );
}
