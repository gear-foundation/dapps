import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';
import { PlayerStatus } from '@/types';

import { Avatar } from '../avatar';

import styles from './player-slot.module.scss';

type Props = {
  avatar?: string;
  position: {
    top?: number;
    left?: number;
    right?: number;
  };
  name: string;
  chips: number;
  status: PlayerStatus;
};

const PlayerSlot = ({ avatar = DefaultAvatar, position, name, chips, status }: Props) => {
  return (
    <div className={clsx(styles.playerSlot)} style={{ ...position }}>
      <Avatar avatar={avatar} size="lg" />
      <div className={styles.playerInfo}>
        <div className={styles.playerName}>{name}</div>
        <div className={styles.playerChips}>{chips}</div>
      </div>
      <div className={styles.playerStatus}>{status}</div>
    </div>
  );
};

export { PlayerSlot };
