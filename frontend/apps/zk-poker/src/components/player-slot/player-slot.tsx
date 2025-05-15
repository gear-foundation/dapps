import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';
import { PlayerStatus } from '@/types';

import { Avatar } from '../avatar';
import { GameTimer } from '../game-timer';

import styles from './player-slot.module.scss';

type Props = {
  avatar?: string;
  top: number;
  name: string;
  chips: number;
  status: PlayerStatus;
  side: 'left' | 'right' | 'center';
};

const PlayerSlot = ({ avatar = DefaultAvatar, top, name, chips, status, side }: Props) => {
  console.log('🚀 ~ PlayerSlot ~ side:', side);
  return (
    <div className={clsx(styles.playerSlot, styles[side])} style={{ top }}>
      {status === 'thinking' && <div className={clsx(styles.highlight, styles[side])} />}
      {status === 'thinking' ? <GameTimer /> : <Avatar avatar={avatar} size="lg" />}
      <div className={styles.playerInfo}>
        <div className={styles.playerName}>{name}</div>
        <div className={styles.playerChips}>{chips}</div>
      </div>
      <div className={styles.playerStatus}>
        {status}
        {status === 'thinking' && '...'}
      </div>
    </div>
  );
};

export { PlayerSlot };
