import clsx from 'clsx';
import { useAtomValue } from 'jotai';
import Countdown, { CountdownRenderProps } from 'react-countdown';

import { BaseComponentProps } from '@/app/types';
import { GameInstance } from '@/app/utils';

import { useGame } from '../../hooks';
import { useConfigQuery } from '../../sails';
import { stateChangeLoadingAtom } from '../../store';
import { GameMark } from '../game-mark';

import styles from './game-countdown.module.scss';

type GameCountdownProps = BaseComponentProps & {
  game: GameInstance;
};

function Clock({ minutes, seconds }: CountdownRenderProps) {
  return (
    <span>
      {`${minutes > 9 ? minutes : '0' + minutes}`}:{seconds > 9 ? seconds : '0' + seconds}
    </span>
  );
}

export function GameCountdown({ game: { player_mark, last_time }, className }: GameCountdownProps) {
  const { setCountdown, countdown } = useGame();
  const { config } = useConfigQuery();
  const isLoading = useAtomValue(stateChangeLoadingAtom);

  return (
    <div className={clsx(styles.wrapper, className)}>
      <div>
        <GameMark mark={player_mark} className={styles.mark} />
      </div>
      <div className={styles.text}>Your turn</div>
      {!isLoading && countdown?.isActive && config && (
        <div className={styles.timer}>
          <Countdown
            date={Number(last_time) + (Number(config.turn_deadline_ms) || 30000)}
            renderer={Clock}
            onComplete={() =>
              setCountdown((prev) => ({
                value: prev ? prev.value : 0,
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
