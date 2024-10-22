import { useNavigate } from 'react-router-dom';
import { Button } from '@gear-js/vara-ui';
import { Modal } from '@/components';
import styles from './game-canceled-modal.module.scss';
import { ROUTES } from '@/app/consts';
import { useSetAtom } from 'jotai';
import { isBattleCanceledAtom } from '@/features/game/store';

export const GameCanceledModal = () => {
  const navigate = useNavigate();
  // const { isCanceled, setIsCanceled } = useEventRegisterCanceledSubscription(gameId);
  const setIsCanceled = useSetAtom(isBattleCanceledAtom);
  const onClose = () => {
    setIsCanceled(false);
    navigate(ROUTES.HOME);
  };

  return (
    <Modal
      title="The game has been canceled by the administrator"
      description="Game administrator has ended the game. All spent VARA tokens for the entry fee will be refunded."
      className={styles.modal}
      onClose={onClose}>
      <Button text="OK" color="grey" className={styles.button} onClick={onClose} />
    </Modal>
  );
};
