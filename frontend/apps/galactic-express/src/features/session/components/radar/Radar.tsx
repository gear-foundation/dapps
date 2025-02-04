import { PLAYER_COLORS } from '@/features/session/consts';
import { CSSProperties } from 'react';
import cropEarthSrc from '../../assets/earth-crop.gif';
import { Event, RankWithName } from '../../types';
import styles from './Radar.module.scss';
import { WinStatus } from '../win-status';

type Props = {
  currentEvents: Event[] | undefined;
  currentRound: number;
  roundsCount: number;
  isWinner: boolean;
  userRank: string;
  winners: RankWithName[];
  admin: string | undefined;
};

function Radar({ currentEvents, currentRound, roundsCount, isWinner, userRank, winners, admin }: Props) {
  const defineHeightIndex = (current: number, firstDead: number) => {
    if (firstDead !== -1) {
      if (current < firstDead) {
        return current;
      }
      return firstDead;
    }

    return current;
  };
  const getPlayers = () =>
    currentEvents?.map(({ participant, deadRound, firstDeadRound }, index) => {
      const playerNumber = index + 1;
      const heightMultiplier = `100% / ${roundsCount}`;
      const heightIndex = defineHeightIndex(currentRound, firstDeadRound);
      const transitionTime = 0.15 * playerNumber;

      const style = {
        opacity: deadRound && '0.3',
        transition: `all ${transitionTime}s`,
        bottom: `calc(${heightMultiplier} * ${heightIndex} + 26px)`,
        left: `calc((100% - (26px * 4)) / 5 * ${playerNumber} + 26px * ${index})`,
        '--color': PLAYER_COLORS[index],
      };

      return <div key={participant} className={styles.player} style={style as CSSProperties} />;
    });

  return (
    <div className={styles.container}>
      {isWinner ? (
        <WinStatus type="win" userRank={userRank} winners={winners} admin={admin} />
      ) : (
        <WinStatus type="lose" userRank={userRank} winners={winners} admin={admin} />
      )}
      <div className={styles.field}>{getPlayers()}</div>

      <img src={cropEarthSrc} alt="" className={styles.image} />
    </div>
  );
}

export { Radar };
