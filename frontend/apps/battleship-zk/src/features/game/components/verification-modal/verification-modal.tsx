import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { ModalBottom } from '@/components/ui/modal';
import styles from './VerificationModal.module.scss';

type Props = {
  isDeadShip?: boolean;
  onVerifyHit: () => void;
  onExit: () => void;
  isLoading: boolean;
};

export default function VerificationModal({ isDeadShip, onVerifyHit, isLoading, onExit }: Props) {
  const heading = `Your ship has been ${isDeadShip ? 'destroyed' : 'hit'}!`;
  const [isCloseConfirmOpen, setIsCloseConfirmOpen] = useState(false);

  return (
    <>
      <ModalBottom heading={heading} onClose={() => setIsCloseConfirmOpen(true)}>
        <div className={styles.content}>
          <div className={styles.buttons}>
            <Button color="primary" text="Confirm" onClick={onVerifyHit} isLoading={isLoading} />
          </div>
        </div>
      </ModalBottom>
      {isCloseConfirmOpen && (
        <ModalBottom heading="Are you sure you want to quit the game?" onClose={() => setIsCloseConfirmOpen(false)}>
          <div className={styles.content}>
            <div className={styles.buttons}>
              <Button color="dark" text="Confirm" onClick={() => onExit()} isLoading={isLoading} />
            </div>
          </div>
        </ModalBottom>
      )}
    </>
  );
}
