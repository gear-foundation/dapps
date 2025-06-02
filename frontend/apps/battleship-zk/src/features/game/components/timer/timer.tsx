import clsx from 'clsx';
import { useEffect, useRef, useState } from 'react';

import { getFormattedTime } from '../../utils';

import styles from './timer.module.scss';

type Props = {
  remainingTime: string | number | bigint | null | undefined;
  shouldGoOn: boolean;
  redOnLast?: boolean;
};

const TIME_LEFT_GAP = 1;

export function Timer({ remainingTime, shouldGoOn, redOnLast }: Props) {
  const [timeLeft, setTimeLeft] = useState<number | null>(null);
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    if (remainingTime === undefined) {
      setTimeLeft(null);
      startTimeRef.current = null;
    } else if (remainingTime === 0) {
      setTimeLeft(0);
    } else {
      const updateTimer = () => {
        if (!shouldGoOn) {
          return;
        }
        const currentTime = new Date().getTime();
        if (startTimeRef.current === null) {
          startTimeRef.current = currentTime;
        }
        const timeLeftMilliseconds =
          Number(remainingTime) + (startTimeRef.current || currentTime) - currentTime - TIME_LEFT_GAP;

        setTimeLeft(Math.max(timeLeftMilliseconds, 0));
      };

      const timerInterval = setInterval(updateTimer, 1000);

      return () => {
        clearInterval(timerInterval);
      };
    }
  }, [shouldGoOn, remainingTime]);

  const displayedTime = timeLeft ?? (remainingTime ? Math.max(Number(remainingTime), 0) : null);
  const isRed = redOnLast && displayedTime !== null ? displayedTime < 10000 : false;
  const formattedTimeLeft = displayedTime !== null ? getFormattedTime(displayedTime, false) : '';

  return <span className={clsx(isRed && styles.red)}>{formattedTimeLeft}</span>;
}
