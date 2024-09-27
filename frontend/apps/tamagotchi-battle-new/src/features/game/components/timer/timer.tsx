import clsx from 'clsx';
import { useEffect, useRef, useState } from 'react';
import styles from './timer.module.scss';
import { Text } from '@/components';

type Props = {
  remainingTime: string | number | bigint | null | undefined;
  shouldGoOn: boolean;
  isYourTurn?: boolean;
};

const TIME_LEFT_GAP = 1;

export function Timer({ remainingTime, shouldGoOn, isYourTurn }: Props) {
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
  const formattedTimeLeft = displayedTime !== null ? Math.round(displayedTime / 1000) : '';

  return (
    <div className={styles.container}>
      <div className={clsx(styles.light, { [styles.dark]: !isYourTurn })} />
      <Text size="xl" weight="bold" className={styles.time}>
        {formattedTimeLeft}
      </Text>

      {isYourTurn ? (
        <>
          <Text size="xl" weight="bold" className={styles.title}>
            YOUR TURN
          </Text>

          <Text size="xs" weight="medium">
            If you don’t act in 60 seconds,
          </Text>
          <Text size="xs" weight="medium">
            “Attack” will be chosen automatically.
          </Text>
        </>
      ) : (
        <Text size="xl" weight="bold" className={styles.title}>
          Waiting for opponent’s turn.
        </Text>
      )}
    </div>
  );
}
