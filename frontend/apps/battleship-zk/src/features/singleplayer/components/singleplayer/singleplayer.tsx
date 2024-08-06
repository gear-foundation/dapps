import { GameProcess, ShipArrangement } from '@/features/game';
import { useArrangementWithSingleplayer } from '../../hooks/use-arrangement-with-singleplayer';
import { useInitSingleGame, useSingleplayerGame } from '../../hooks/use-singleplayer-game';
import { useProcessWithSingleplayer } from '../../hooks/use-process-with-singleplayer';
import { useEventMoveMadeSubscription } from '../../sails/events/use-event-move-made-subscription';

export function Singleplayer() {
  const gameType = 'single';
  const { makeStartGameTransaction } = useArrangementWithSingleplayer();
  const {
    totalShoots,
    successfulShoots,
    gameEndResult,
    remainingTime,
    gameUpdatedEvent,
    handleClickCell,
    exitGame,
    verifyOponentsHit,
  } = useProcessWithSingleplayer();
  const { isActiveGame, triggerGame, resetGameState } = useSingleplayerGame();

  useInitSingleGame();
  useEventMoveMadeSubscription();

  return isActiveGame ? (
    <GameProcess
      gameType={gameType}
      totalShoots={totalShoots}
      successfulShoots={successfulShoots}
      gameResults={gameEndResult ? { totalTime: gameEndResult.time, winner: gameEndResult.winner } : null}
      remainingTime={remainingTime}
      gameUpdatedEvent={gameUpdatedEvent}
      admin={undefined}
      onClickCell={handleClickCell}
      onExitGame={exitGame}
      onVerifyOponentsHit={verifyOponentsHit}
      resetGameState={resetGameState}
    />
  ) : (
    <ShipArrangement
      gameType={gameType}
      makeStartGameTransaction={makeStartGameTransaction}
      triggerGame={triggerGame}
    />
  );
}
