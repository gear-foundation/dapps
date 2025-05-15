import clsx from 'clsx';
import { useState } from 'react';

import { GameTimer } from '../game-timer';

import styles from './your-turn.module.scss';

type Props = {
  className?: string;
  timeoutSec?: number;
};

const YourTurn = ({ className, timeoutSec = 30 }: Props) => {
  const handleTimeEnd = () => {
    console.log('Time ended');
  };

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
        timeoutSec={timeoutSec}
        onTimeEnd={handleTimeEnd}
        onTenSecondsLeft={handleTenSecondsLeft}
        className={styles.timer}
      />
    </>
  );
};

export { YourTurn };
