import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';

import { ModalBottom } from '@/components/ui/modal';
import { Text } from '@/components/ui/text';

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
      {!isCloseConfirmOpen && (
        <ModalBottom heading={heading} onClose={() => setIsCloseConfirmOpen(true)}>
          <div className={styles.content}>
            <div className={styles.buttons}>
              <Button color="primary" text="Confirm" onClick={onVerifyHit} isLoading={isLoading} />
            </div>
            <Text>
              The arrangement of ships is not stored in the on-chain program. By clicking the Confirm button, the ZK
              proof will be generated to confirm whether your ship has been hit or destroyed.
            </Text>
          </div>
        </ModalBottom>
      )}
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
