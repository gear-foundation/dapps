import { Text } from '@/components/ui/text';
import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { ModalBottom } from '@/components/ui/modal';
import styles from './GameCancelledModal.module.scss';

type Props = {
  text: string;
  onClose: () => void;
};

export default function GameCancelledModal({ text, onClose }: Props) {
  return (
    <ModalBottom heading="Game canceled" onClose={onClose}>
      <div className={styles.content}>
        <Text>{text}</Text>
        <div className={styles.buttons}>
          <Button color="dark" text="Exit" onClick={onClose} />
        </div>
      </div>
    </ModalBottom>
  );
}
