import { Button } from '@gear-js/vara-ui';
import { useGaslessTransactions } from 'gear-ez-transactions';
import { useEffect, useState } from 'react';

import { useCountdown } from '@dapps-frontend/hooks';

import { ModalBottom } from '@/components/ui/modal';
import { Text } from '@/components/ui/text';

import styles from './VoucherExpiredModal.module.scss';

export default function VoucherExpiredModal() {
  const { expireTimestamp, setIsEnabled } = useGaslessTransactions();
  const [isOpen, setIsOpen] = useState(true);

  const countdown = useCountdown(expireTimestamp || undefined);
  const isVoucherExpired = countdown === 0;

  useEffect(() => {
    if (isVoucherExpired) {
      setIsOpen(true);
      setIsEnabled(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isVoucherExpired]);

  return isOpen && isVoucherExpired ? (
    <ModalBottom heading="Voucher Expired" onClose={() => setIsOpen(false)}>
      <div className={styles.content}>
        <Text>Your voucher has expired and couldn&apos;t be used.</Text>
        <div className={styles.buttons}>
          <Button color="contrast" text="Exit" onClick={() => setIsOpen(false)} />
        </div>
      </div>
    </ModalBottom>
  ) : null;
}
