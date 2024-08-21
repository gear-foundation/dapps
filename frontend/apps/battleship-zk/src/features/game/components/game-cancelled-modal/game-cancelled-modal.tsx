import { Text } from '@/components/ui/text';
import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { ModalBottom } from '@/components/ui/modal';
import styles from './GameCancelledModal.module.scss';

type Props = {
  isOpen: boolean;
  text: string;
  onClose: () => void;
};

export default function GameCancelledModal({ isOpen, text, onClose }: Props) {
  return isOpen ? (
    <ModalBottom heading="Game canceled" onClose={onClose}>
      <div className={styles.content}>
        <Text>{text}</Text>
        <div className={styles.buttons}>
          <Button color="dark" text="Exit" onClick={onClose} />
        </div>
      </div>
    </ModalBottom>
  ) : null;
}
