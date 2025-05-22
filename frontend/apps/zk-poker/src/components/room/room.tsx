import clsx from 'clsx';

import { ChipsIcon, MemberIcon, TimeIcon, UserIcon } from '@/assets/images';

import { Avatar } from '../avatar';

import styles from './room.module.scss';

type Props = {
  name: string;
  totalPlayers: number;
  currentPlayers: number;
  buyIn: number;
  adminName: string;
  time: number;
};

const Room = ({ name, totalPlayers, currentPlayers, buyIn, time, adminName }: Props) => {
  const formattedBuyIn = String(buyIn).slice(0, -3);
  const haveSeat = currentPlayers !== totalPlayers;

  return (
    <div className={styles.container}>
      <Avatar className={styles.avatar} />
      <div className={styles.roomInfo}>
        <div className={styles.roomHeader}>
          <h3 className={styles.roomTitle}>{name}</h3>
          <div className={clsx(styles.buyIn, !haveSeat && styles.disabled)}>
            <ChipsIcon />
            <span>Buy in {formattedBuyIn}k PTS</span>
          </div>
        </div>
        <div className={styles.footer}>
          <div className={styles.info}>
            <MemberIcon />
            <span>
              <span className={clsx(haveSeat && styles.haveSeat)}>{currentPlayers}</span>/{totalPlayers}
            </span>
          </div>
          <div className={styles.info}>
            <TimeIcon />
            <span>{time} sec</span>
          </div>
          <div className={styles.info}>
            <UserIcon />
            <span>{adminName}</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export { Room };
