import { Modal } from 'components/layout/modal';
import { Button } from '@gear-js/vara-ui';
import styles from './GameCancelledModal.module.scss';

type Props = {
  admin: string;
  onClose: () => void;
};

function GameCancelledModal({ admin, onClose }: Props) {
  return (
    <Modal
      heading="The game has been canceled 
    by the administrator"
      className={{ header: styles.modalHeader }}
      onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.text}>
          Game administrator {admin} has ended the game. All spent VARA tokens for the entry fee will be refunded.
        </p>

        <Button text="OK" color="grey" className={styles.button} onClick={onClose} />
      </div>
    </Modal>
  );
}

export { GameCancelledModal };
