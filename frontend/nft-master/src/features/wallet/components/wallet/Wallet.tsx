import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { AccountIcon, Button } from 'components';
import clsx from 'clsx';
import { WalletModal } from '../wallet-modal';
import styles from './Wallet.module.scss';

function Wallet({ className }: { className?: string }) {
  const { account, isAccountReady } = useAccount();
  const [open, setOpen] = useState(false);

  const openWalletModal = () => setOpen(true);
  const closeWalletModal = () => setOpen(false);

  return isAccountReady ? (
    <>
      <Button
        variant={account ? 'black' : 'primary'}
        className={clsx(styles.button, className)}
        onClick={openWalletModal}>
        {account && <AccountIcon value={account.address} size={16} className={styles.icon} />}
        <span>{account ? account.meta.name : 'Connect'}</span>
      </Button>

      {open && <WalletModal onClose={closeWalletModal} />}
    </>
  ) : null;
}

export { Wallet };
