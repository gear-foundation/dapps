import { GameProcess, ShipArrangement } from '@/features/game';
import { useGame, useInitGame } from '@/features/game/hooks';

export default function GamePage() {
  useInitGame();
  const { isActiveGame } = useGame();

  return <div>{isActiveGame ? <GameProcess /> : <ShipArrangement />}</div>;
}
