import clsx from 'clsx';

import styles from './lobby-timer.module.scss';

type Props = {
  remainingMs: number;
  isBeforeStart?: boolean;
  className?: string;
};

export const LobbyTimer = ({ remainingMs, isBeforeStart, className }: Props) => {
  const totalSeconds = Math.floor(remainingMs / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;

  if (isBeforeStart) {
    return (
      <div className={clsx(styles.beforeStartWrapper, className)}>
        <span className={styles.label}>Game starts in</span>
        <span className={styles.time}>
          {minutes}:{seconds.toString().padStart(2, '0')}
        </span>
      </div>
    );
  }

  return (
    <div className={clsx(styles.wrapper, className)}>
      <span className={styles.label}>Lobby time left</span>
      <span className={styles.time}>
        {minutes}:{seconds.toString().padStart(2, '0')}
      </span>
    </div>
  );
};
