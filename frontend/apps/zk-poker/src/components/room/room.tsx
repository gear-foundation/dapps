import { HexString } from '@gear-js/api';
import clsx from 'clsx';
import { Link } from 'react-router-dom';

import { useCountdown } from '@dapps-frontend/hooks';

import { MAX_PLAYERS } from '@/app/consts';
import { ChipsIcon, MemberIcon, TimeIcon, UserIcon } from '@/assets/images';
import { LobbyTimer } from '@/features/game/components';

import { Avatar } from '../avatar';

import styles from './room.module.scss';

type StatusVariant = 'live' | 'waiting' | 'completed';

const getStatusDisplay = (status: string): { label: string; variant: StatusVariant } => {
  if (status === 'created') return { label: 'Open', variant: 'waiting' };
  if (status === 'finished') return { label: 'Closed', variant: 'completed' };
  return { label: 'Live', variant: 'live' };
};

type Props = {
  name: string;
  currentPlayersCount: number;
  buyIn: number;
  adminName: string;
  adminId: HexString;
  time: number;
  id: HexString;
  status: string;
  timeUntilStartMs?: number;
};

const Room = ({ name, currentPlayersCount, buyIn, time, adminName, adminId, id, status, timeUntilStartMs }: Props) => {
  const formattedBuyIn = String(buyIn).slice(0, -3);
  const haveSeat = currentPlayersCount !== MAX_PLAYERS;
  const { label, variant } = getStatusDisplay(status);
  const timeLeftMs = useCountdown(timeUntilStartMs);

  return (
    <Link to={`/game/${id}`}>
      <div className={styles.container}>
        <Avatar className={styles.avatar} address={adminId} />
        <div className={styles.statusContainer}>
          <div className={clsx(styles.status, styles[`status_${variant}`])}>{label}</div>
        </div>

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
          {Boolean(timeLeftMs) && (
            <div className={styles.info}>
              <LobbyTimer remainingMs={timeLeftMs ?? 0} isBeforeStart className={styles.timer} />
            </div>
          )}
        </div>
      </div>
    </Link>
  );
};

export { Room };
