import { HexString } from '@gear-js/api';
import clsx from 'clsx';

import { PlayerStatus } from '@/features/zk/api/types';

import { Avatar } from '../avatar';
import { GameTimer } from '../game-timer';

import styles from './player-slot.module.scss';

type Props = {
  address?: HexString;
  top?: number;
  name: string;
  chips: number;
  status: PlayerStatus;
  bet?: number;
  side: 'left' | 'right' | 'top' | 'bottom';
  hideAvatar?: boolean;
  timePerMoveMs: number;
};

const PlayerSlot = ({ address, top, name, chips, status, side, bet, hideAvatar, timePerMoveMs }: Props) => {
  return (
    <div className={clsx(styles.playerSlot, styles[side])} style={{ top }}>
      {status === 'thinking' && <div className={clsx(styles.highlight, styles[side])} />}

      {hideAvatar && <Avatar isHidden size="lg" />}
      {!hideAvatar &&
        (status === 'thinking' ? (
          <GameTimer timeoutSec={timePerMoveMs / 1000} />
        ) : (
          <Avatar address={address} size="lg" />
        ))}

      <div className={styles.playerInfo}>
        <div className={styles.playerName}>{name}</div>
        <div className={styles.playerChips}>{chips}</div>
      </div>
      <div className={clsx(styles.playerStatus, styles[status])}>
        {status}
        {status === 'thinking' && '...'}
        {status === 'bet' && <span> {bet}</span>}
      </div>
    </div>
  );
};

export { PlayerSlot };
