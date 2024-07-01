import { useInitMultiplayerGame } from '@/features/multiplayer/hooks';
import { Singleplayer } from '@/features/singleplayer/components/singleplayer';
import { Multiplayer } from '@/features/multiplayer/components/multiplayer';
import styles from './game.module.scss';

export default function GamePage() {
  const { isActiveGame: isActiveMultiplayer } = useInitMultiplayerGame();

  return (
    <>
      <div className={styles.gameDarkHeading} />

      <div className={styles.gameContainer}>{isActiveMultiplayer ? <Multiplayer /> : <Singleplayer />}</div>
    </>
  );
}
