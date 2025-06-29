import clsx from 'clsx';
import { useEffect, useState } from 'react';

import styles from './game-timer.module.scss';

type Props = {
  timeoutSec: number;
  className?: string;
  onTimeEnd?: () => void;
  onTenSecondsLeft?: () => void;
  size?: 'lg' | 'md';
};

const GameTimer = ({ className, timeoutSec, onTimeEnd, onTenSecondsLeft, size = 'md' }: Props) => {
  const [timeLeft, setTimeLeft] = useState(timeoutSec);

  useEffect(() => {
    const timer = setInterval(() => {
      setTimeLeft((prev) => {
        if (prev <= 10) {
          if (onTenSecondsLeft) onTenSecondsLeft();
        }
        if (prev <= 1) {
          clearInterval(timer);
          if (onTimeEnd) onTimeEnd();
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => clearInterval(timer);
  }, [onTimeEnd, timeoutSec, onTenSecondsLeft]);

  const radius = size === 'lg' ? 47 : 45; // %
  const circumference = 2 * Math.PI * radius;
  const progressPercentage = (timeLeft / timeoutSec) * 100;
  const dashOffset = circumference - (progressPercentage / 100) * circumference;

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className={clsx(styles.timerContainer, styles[size], className)}>
      <div className={styles.timerInner}>
        <svg className={styles.timerSvg} viewBox="0 0 100 100">
          <circle className={styles.timerBackground} cx="50" cy="50" r={radius} fill="transparent" />
          <circle
            className={styles.timerProgress}
            cx="50"
            cy="50"
            r={radius}
            fill="transparent"
            strokeDasharray={circumference}
            strokeDashoffset={dashOffset}
            transform="rotate(90, 50, 50)"
          />
        </svg>
        <div className={styles.timerText}>{formatTime(timeLeft)}</div>
      </div>
    </div>
  );
};

export { GameTimer };
