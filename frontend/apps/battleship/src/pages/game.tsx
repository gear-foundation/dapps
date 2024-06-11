import { GameProcess, ShipArrangement } from '@/features/game';
import { useGame, useInitGame } from '@/features/game/hooks';
import styles from './game.module.scss';

export default function GamePage() {
  useInitGame();
  const { isActiveGame } = useGame();

  return (
    <>
      <div className={styles.gameDarkHeading} />
      <div className={styles.gameContainer}>{isActiveGame ? <GameProcess /> : <ShipArrangement />}</div>
    </>
  );
}
