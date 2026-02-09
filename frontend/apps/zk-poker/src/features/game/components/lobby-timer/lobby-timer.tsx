import styles from './lobby-timer.module.scss';

type Props = {
  remainingMs: number;
  isBeforeStart?: boolean;
};

export const LobbyTimer = ({ remainingMs, isBeforeStart }: Props) => {
  const totalSeconds = Math.floor(remainingMs / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;

  if (isBeforeStart) {
    return (
      <div className={styles.beforeStartWrapper}>
        <span className={styles.label}>Game starts in</span>
        <span className={styles.time}>
          {minutes}:{seconds.toString().padStart(2, '0')}
        </span>
      </div>
    );
  }

  return (
    <div className={styles.wrapper}>
      <span className={styles.label}>Lobby time left</span>
      <span className={styles.time}>
        {minutes}:{seconds.toString().padStart(2, '0')}
      </span>
    </div>
  );
};
