import { PLAYER_COLORS } from 'features/session/consts';
import { CSSProperties } from 'react';
import cropEarthSrc from '../../assets/earth-crop.gif';
import { Event } from '../../types';
import styles from './Radar.module.scss';
import { WinStatus } from '../win-status';

type Props = {
  currentEvents: Event[] | undefined;
  currentRound: number;
  roundsCount: number;
  isWinner: boolean;
  userRank: number;
};

function Radar({ currentEvents, currentRound, roundsCount, isWinner, userRank }: Props) {
  console.log(roundsCount);
  const getPlayers = () =>
    currentEvents?.map(({ participant, deadRound }, index) => {
      const playerNumber = index + 1;
      const heightMultiplier = `100% / ${roundsCount}`;
      const heightIndex = deadRound || currentRound;
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
      {isWinner ? <WinStatus type="win" userRank={userRank} /> : <WinStatus type="lose" userRank={userRank} />}
      <div className={styles.field}>{getPlayers()}</div>

      <img src={cropEarthSrc} alt="" className={styles.image} />
    </div>
  );
}

export { Radar };
