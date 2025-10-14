import { HexString } from '@gear-js/api';
import clsx from 'clsx';
import { Link } from 'react-router-dom';

import { MAX_PLAYERS } from '@/app/consts';
import { ChipsIcon, MemberIcon, TimeIcon, UserIcon } from '@/assets/images';

import { Avatar } from '../avatar';

import styles from './room.module.scss';

type Props = {
  name: string;
  currentPlayersCount: number;
  buyIn: number;
  adminName: string;
  adminId: HexString;
  time: number;
  id: HexString;
};

const Room = ({ name, currentPlayersCount, buyIn, time, adminName, adminId, id }: Props) => {
  const formattedBuyIn = String(buyIn).slice(0, -3);
  const haveSeat = currentPlayersCount !== MAX_PLAYERS;

  return (
    <Link to={`/game/${id}`}>
      <div className={styles.container}>
        <Avatar className={styles.avatar} address={adminId} />
        <div className={styles.roomInfo}>
          <div className={styles.roomHeader}>
            <h3 className={styles.roomTitle}>{name}</h3>
            <div className={styles.buyIn}>
              <ChipsIcon />
              <span>Buy in {formattedBuyIn}k PTS</span>
            </div>
          </div>
          <div className={styles.footer}>
            <div className={styles.info}>
              <MemberIcon />
              <span>
                <span className={clsx(haveSeat && styles.haveSeat)}>{currentPlayersCount}</span>/{MAX_PLAYERS}
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
