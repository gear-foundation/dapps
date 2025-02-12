import { Button } from '@gear-js/vara-ui';
import { useSetAtom } from 'jotai';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { Modal } from '@/components';
import { isBattleCanceledAtom } from '@/features/game/store';

import styles from './game-canceled-modal.module.scss';

export const GameCanceledModal = () => {
  const navigate = useNavigate();
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
