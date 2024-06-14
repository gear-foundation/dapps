import { GameProcess, ShipArrangement } from '@/features/game';
import { useGame, useInitGame } from '@/features/game/hooks';
import { useEventMoveMadeSubscription } from '@/app/utils/sails/events/use-event-move-made-subscription';
import styles from './game.module.scss';

export default function GamePage() {
  useInitGame();
  useEventMoveMadeSubscription();

  const { isActiveGame } = useGame();

  return (
    <>
      <div className={styles.gameDarkHeading} />
      <div className={styles.gameContainer}>{isActiveGame ? <GameProcess /> : <ShipArrangement />}</div>
    </>
  );
}
