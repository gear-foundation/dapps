import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import Identicon from '@polkadot/react-identicon';
import { Button } from 'components';
import clsx from 'clsx';
import { useIsAppReady } from 'app/hooks/use-is-app-ready';
import { WalletModal } from '../wallet-modal';
import styles from './Wallet.module.scss';

export function Wallet({ className }: { className?: string }) {
  const [open, setOpen] = useState(false);

  const { account } = useAccount();
  const { isAppReady } = useIsAppReady();

  const isSigned = isAppReady && !!account;

  return (
    <>
      <Button
        variant={isSigned ? 'black' : 'primary'}
        className={clsx(styles.button, className)}
        onClick={() => setOpen(true)}
        disabled={!isAppReady}>
        {isSigned && <Identicon value={account.address} size={16} theme="polkadot" className={styles.icon} />}
        <span>{isSigned ? account.meta.name : 'Connect'}</span>
      </Button>

      {open && <WalletModal onClose={() => setOpen(false)} />}
    </>
  );
}
