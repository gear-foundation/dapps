import clsx from 'clsx';

import { Text } from '@/components';

import { UseTimerParams, useTimer } from '../../hooks';

import styles from './timer.module.scss';

type Props = UseTimerParams & {
  isYourTurn?: boolean;
};

export function Timer({ remainingTime, shouldGoOn, isYourTurn }: Props) {
  const formattedTimeLeft = useTimer({ remainingTime, shouldGoOn });

  return (
    <div className={styles.container}>
      <div className={clsx(styles.light, { [styles.dark]: !isYourTurn })} />
      <p className={styles.time}>{formattedTimeLeft}</p>

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
