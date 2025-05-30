import { HexString } from '@gear-js/api';
import clsx from 'clsx';
import { Link } from 'react-router-dom';

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
  id: HexString;
};

const Room = ({ name, totalPlayers, currentPlayers, buyIn, time, adminName, id }: Props) => {
  const formattedBuyIn = String(buyIn).slice(0, -3);
  const haveSeat = currentPlayers !== totalPlayers;

  return (
    <Link to={`/game/${id}`}>
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
    </Link>
  );
};

export { Room };
