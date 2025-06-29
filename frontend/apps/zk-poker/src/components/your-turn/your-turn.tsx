import clsx from 'clsx';
import { useState } from 'react';

import { GameTimer } from '../game-timer';

import styles from './your-turn.module.scss';

type Props = {
  timePerMoveMs: number;
  className?: string;
  onTimeEnd: () => void;
};

const YourTurn = ({ className, timePerMoveMs, onTimeEnd }: Props) => {
  const [showHint, setShowHint] = useState(false);
  const handleTenSecondsLeft = () => {
    setShowHint(true);
  };

  return (
    <>
      <div className={clsx(styles.container, className)}>
        <div className={styles.textContainer}>
          <span className={styles.text}>Your turn</span>
        </div>
        {showHint && (
          <div className={styles.hint}>If you don’t act before the timer ends, you’ll automatically fold.</div>
        )}
      </div>
      <GameTimer
        size="lg"
        timeoutSec={timePerMoveMs / 1000}
        onTimeEnd={onTimeEnd}
        onTenSecondsLeft={handleTenSecondsLeft}
        className={styles.timer}
      />
    </>
  );
};

export { YourTurn };
