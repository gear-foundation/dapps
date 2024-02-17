import { Modal } from 'components/layout/modal';
import { Button } from '@gear-js/vara-ui';
import styles from './GameNotFoundModal.module.scss';

type Props = {
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function GameNotFoundModal({ onClose }: Props) {
  return (
    <Modal heading="Game not found" className={{ header: styles.modalHeader }} onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.text}>
          Please check the entered address. It&#39;s possible the game has been canceled or does not exist.
        </p>

        <Button text="OK" color="grey" className={styles.button} onClick={onClose} />
      </div>
    </Modal>
  );
}

export { GameNotFoundModal };
