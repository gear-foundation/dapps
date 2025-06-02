import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';

import { ModalBottom } from '@/components/ui/modal';
import { Text } from '@/components/ui/text';

import styles from './YourTurnModal.module.scss';

type Props = {
  isYourTurn: boolean;
};

export default function YourTurnModal({ isYourTurn }: Props) {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <>
      {isOpen && isYourTurn && (
        <ModalBottom heading="It’s your turn" onClose={() => setIsOpen(false)}>
          <div className={styles.content}>
            <Text>You have 1 minute per turn. If no turn is made, you will lose.</Text>
            <div className={styles.buttons}>
              <Button color="primary" text="OK" onClick={() => setIsOpen(false)} />
            </div>
          </div>
        </ModalBottom>
      )}
    </>
  );
}
