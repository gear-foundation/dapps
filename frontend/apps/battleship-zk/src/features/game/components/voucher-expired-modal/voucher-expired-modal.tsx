import { useEffect, useState } from 'react';
import { Text } from '@/components/ui/text';
import { Button } from '@gear-js/vara-ui';
import { useGaslessTransactions } from '@dapps-frontend/ez-transactions';
import { useCountdown } from '@dapps-frontend/hooks';
import { ModalBottom } from '@/components/ui/modal';
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
  }, [isVoucherExpired]);

  return isOpen && isVoucherExpired ? (
    <ModalBottom heading="Voucher Expired" onClose={() => setIsOpen(false)}>
      <div className={styles.content}>
        <Text>Your voucher has expired and couldn't be used.</Text>
        <div className={styles.buttons}>
          <Button color="dark" text="Exit" onClick={() => setIsOpen(false)} />
        </div>
      </div>
    </ModalBottom>
  ) : null;
}
