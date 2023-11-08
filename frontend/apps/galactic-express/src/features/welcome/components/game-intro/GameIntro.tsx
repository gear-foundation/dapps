import { cx } from 'utils';
import styles from './GameIntro.module.scss';

function GameIntro() {
  return (
    <div className={cx(styles.container)}>
      <h1 className={cx(styles.name)}>Rocket Game</h1>
      <p className={cx(styles.description)}>
        To start the game, you need to choose your role. The administrator must have the uploaded smart contract of the
        game and provide its address. The user receives the game address from the administrator to enter the prepared
        game.
      </p>
    </div>
  );
}

export { GameIntro };
