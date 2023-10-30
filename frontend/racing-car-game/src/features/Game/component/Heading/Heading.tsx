import { Spinner } from '@/components';
import { HeadingProps } from './Heading.interface';
import styles from './Heading.module.scss';
import { cx } from '@/utils';

function Heading({ currentTurn, isPlayerAction, winStatus }: HeadingProps) {
  return (
    <div className={cx(styles.heading)}>
      <h1 className={cx(styles['heading-title'], styles[`heading-title-with-gradient-${winStatus}`])}>
        {!winStatus && 'Racing Car Game'}
        {winStatus === 'Win' && 'You Win'}
        {winStatus === 'Draw' && `It's A Draw`}
        {winStatus === 'Lose' && 'You Lose'}
      </h1>
      <h3 className={cx(styles['heading-description'])}>
        {!winStatus && 'Either accelerate or shoot at the nearest car to win.'}
        {winStatus === 'Win' && 'Congratulations, the game is over, you won!'}
        {winStatus === 'Draw' && `The game is over, it's a draw! Try again to win.`}
        {winStatus === 'Lose' && 'Try again to win.'}
      </h3>
      <div className={cx(styles.turn)}>
        <span className={cx(styles['turn-value'])}>Turn {currentTurn}</span>
        {!isPlayerAction && <Spinner />}
      </div>
    </div>
  );
}

export { Heading };
