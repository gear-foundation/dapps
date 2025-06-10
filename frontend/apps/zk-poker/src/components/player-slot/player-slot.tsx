import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';
import { PlayerStatus } from '@/features/zk/api/types';

import { Avatar } from '../avatar';
import { GameTimer } from '../game-timer';

import styles from './player-slot.module.scss';

type Props = {
  avatar?: string;
  top?: number;
  name: string;
  chips: number;
  status: PlayerStatus;
  bet?: number;
  side: 'left' | 'right' | 'top' | 'bottom';
  hideAvatar?: boolean;
};

const PlayerSlot = ({ avatar = DefaultAvatar, top, name, chips, status, side, bet, hideAvatar }: Props) => {
  return (
    <div className={clsx(styles.playerSlot, styles[side])} style={{ top }}>
      {status === 'thinking' && <div className={clsx(styles.highlight, styles[side])} />}

      {hideAvatar && <Avatar isHidden size="lg" />}
      {!hideAvatar && (status === 'thinking' ? <GameTimer /> : <Avatar avatar={avatar} size="lg" />)}

      <div className={styles.playerInfo}>
        <div className={styles.playerName}>{name}</div>
        <div className={styles.playerChips}>{chips}</div>
      </div>
      <div className={clsx(styles.playerStatus, styles[status])}>
        {status}
        {status === 'thinking' && '...'}
        {status === 'bet' && <span> ${bet}</span>}
      </div>
    </div>
  );
};

export { PlayerSlot };
