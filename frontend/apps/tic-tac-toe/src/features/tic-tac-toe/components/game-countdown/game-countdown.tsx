import styles from './game-countdown.module.scss';
import Countdown, { CountdownRenderProps } from 'react-countdown';
import { GameMark } from '../game-mark';
import { useGame } from '../../hooks';
import type { IGameInstance } from '../../types';
import { toNumber } from '@/app/utils';
import clsx from 'clsx';
import { BaseComponentProps } from '@/app/types';
import { useAtomValue } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';

type GameCountdownProps = BaseComponentProps & {
  game: IGameInstance;
};

function Clock({ minutes, seconds }: CountdownRenderProps) {
  return (
    <span>
      {`${minutes > 9 ? minutes : '0' + minutes}`}:{seconds > 9 ? seconds : '0' + seconds}
    </span>
  );
}

export function GameCountdown({ game: { playerMark, lastTime }, className }: GameCountdownProps) {
  const { setCountdown, countdown, configState } = useGame();
  const isLoading = useAtomValue(stateChangeLoadingAtom);

  return (
    <div className={clsx(styles.wrapper, className)}>
      <div>
        <GameMark mark={playerMark} className={styles.mark} />
      </div>
      <div className={styles.text}>Your turn</div>
      {!isLoading && countdown?.isActive && configState && (
        <div className={styles.timer}>
          <Countdown
            date={toNumber(lastTime) + (toNumber(configState.turnDeadlineMs) || 30000)}
            renderer={Clock}
            onComplete={() =>
              setCountdown((prev) => ({
                value: prev ? prev.value : '',
                isActive: false,
              }))
            }
          />
        </div>
      )}
      {isLoading && (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          className={styles.loader}
          width={20}
          height={20}
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round">
          <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
      )}
    </div>
  );
}
