import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from 'components/ui/button';
import { DialogsLibrary } from 'components/ui/dialogs';
import clsx from 'clsx';
import { useAccountAvailableBalance } from 'features/account-available-balance/hooks';
import { VaraBalance } from 'components/ui/balance';
import type { BaseComponentProps } from 'types';
import styles from './Wallet.module.scss';
import { WalletIcon } from '../wallet-icon';

export function Wallet({ className }: BaseComponentProps) {
  const [open, setOpen] = useState(false);

  const { account } = useAccount();
  const { availableBalance: balance } = useAccountAvailableBalance();

  const isSigned = !!account;

  return (
    <>
      <div className={clsx(styles.wrapper, className)}>
        {isSigned && (
          <VaraBalance
            value={balance?.value || '0'}
            unit={balance?.unit}
            className={styles.balance}
          />
        )}
        <Button
          variant={isSigned ? 'black' : 'primary'}
          className={clsx(styles.button, className)}
          onClick={() => setOpen(true)}
        >
          {isSigned && (
            <WalletIcon
              address={account.address}
              size={16}
              className={styles.icon}
            />
          )}
          <span>{isSigned ? account.meta.name : 'Connect Wallet'}</span>
        </Button>
      </div>

      <DialogsLibrary.WalletModal open={open} setOpen={setOpen} />
    </>
  );
}
