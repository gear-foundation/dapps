import { GameProcess, ShipArrangement } from '@/features/game';
import { useArrangementWithSingleplayer } from '../../hooks/use-arrangement-with-singleplayer';
import { useInitSingleGame, useSingleplayerGame } from '../../hooks/use-singleplayer-game';
import { useProcessWithSingleplayer } from '../../hooks/use-process-with-singleplayer';
import { useEventMoveMadeSubscription } from '../../sails/events/use-event-move-made-subscription';

export function Singleplayer() {
  const gameType = 'single';
  const { makeStartGameTransaction } = useArrangementWithSingleplayer();
  const { totalShoots, successfulShoots, gameEndResult, gameStartTime, gameUpdatedEvent, handleClickCell, exitGame } =
    useProcessWithSingleplayer();
  const { isActiveGame } = useSingleplayerGame();

  useInitSingleGame();
  useEventMoveMadeSubscription();

  return isActiveGame ? (
    <GameProcess
      gameType={gameType}
      totalShoots={totalShoots}
      successfulShoots={successfulShoots}
      gameResults={gameEndResult ? { totalTime: gameEndResult.time, winner: gameEndResult.winner } : null}
      gameStartTime={gameStartTime}
      gameUpdatedEvent={gameUpdatedEvent}
      onClickCell={handleClickCell}
      onExitGame={exitGame}
    />
  ) : (
    <ShipArrangement gameType={gameType} makeStartGameTransaction={makeStartGameTransaction} />
  );
}
