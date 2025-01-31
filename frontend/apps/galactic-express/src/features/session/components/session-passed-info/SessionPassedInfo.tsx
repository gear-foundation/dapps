import { useSetAtom } from 'jotai';
import { Button } from '@gear-js/vara-ui';
import { CURRENT_GAME_ATOM } from '@/atoms';
import styles from './SessionPassedInfo.module.scss';

function SessionPassedInfo() {
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);

  const handleClick = () => {
    setCurrentGame(null);
  };

  return (
    <div className={styles.sessionPassedInfoWrapper}>
      <div>The session has passed. You are not participating in this one</div>
      <Button text="Cancel" onClick={handleClick} className={styles.button} />
    </div>
  );
}

export { SessionPassedInfo };
