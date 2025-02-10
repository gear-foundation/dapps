import { decodeAddress } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { Modal } from '@/components';
import { GameProcess, ShipArrangement } from '@/features/game';
import { GameCancelledModal } from '@/features/game/components';
import { usePending } from '@/features/game/hooks';
import { useShips } from '@/features/zk/hooks/use-ships';

import { useMultiplayerGame, useProcessWithMultiplayer, useArrangementWithMultiplayer } from '../../hooks';
import {
  useEventPlacementVerified,
  useEventGameCancelled,
  useEventMoveMadeSubscription,
  useEventPlayerDeleted,
  useEventGameLeft,
} from '../../sails/events';
import { getIsPlacementStatus } from '../../utils';

import styles from './Multiplayer.module.scss';

export function Multiplayer() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { gameType, makeStartGameTransaction } = useArrangementWithMultiplayer();
  const {
    totalShoots,
    successfulShoots,
    gameEndResult,
    remainingTime,
    gameUpdatedEvent,
    handleClickCell,
    exitGame,
    verifyOponentsHit,
  } = useProcessWithMultiplayer();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();
  const { getBoard } = useShips();
  const { pending } = usePending();
  const [savedPlayerBoard, setSavedPlayerBoard] = useState<string[] | null | undefined>();

  useEventPlacementVerified();
  const { isGameCancelled, onGameCancelled } = useEventGameCancelled();
  useEventMoveMadeSubscription();
  const { isPlayerDeleted, onPlayerDeleted } = useEventPlayerDeleted();
  const { isGameLeft, onGameLeft } = useEventGameLeft();

  const playerInfo = game?.participants_data.find((item) => decodeAddress(item[0]) === account?.decodedAddress)?.[1];
  const isPlacementStatus = getIsPlacementStatus(game);

  const handleCloseModal = () => {
    navigate(ROUTES.HOME);
  };

  const loadSavedBoard = () => {
    setSavedPlayerBoard(getBoard(gameType, 'player'));
  };

  useEffect(() => {
    loadSavedBoard();
  }, []);

  return isPlacementStatus ? (
    <>
      <ShipArrangement
        gameType={gameType}
        savedBoard={savedPlayerBoard}
        makeStartGameTransaction={makeStartGameTransaction}
        triggerGame={triggerGame}
      />
      {playerInfo?.ship_hash && playerInfo.ship_hash.toString() !== '0x' && (
        <Modal heading="Please Wait" onClose={handleCloseModal} closeOnMissclick={false}>
          <div className={styles.waitModalContent}>
            The opponent hasn't placed their ships yet.
            <Button disabled={pending} className={styles.leaveGameButton} onClick={exitGame}>
              Leave game
            </Button>
          </div>
        </Modal>
      )}

      {isPlayerDeleted && (
        <GameCancelledModal
          text={'You have been removed from the game by an administrator.'}
          onClose={onPlayerDeleted}
        />
      )}
      {isGameCancelled && (
        <GameCancelledModal text={'The game was terminated by the administrator.'} onClose={onGameCancelled} />
      )}
      {isGameLeft && <GameCancelledModal text={'Your opponent has left the game.'} onClose={onGameLeft} />}
    </>
  ) : (
    <GameProcess
      gameType={gameType}
      totalShoots={totalShoots}
      successfulShoots={successfulShoots}
      gameResults={gameEndResult ? { totalTime: gameEndResult.total_time, winner: gameEndResult.winner } : null}
      remainingTime={remainingTime}
      gameUpdatedEvent={gameUpdatedEvent}
      admin={game?.admin}
      onClickCell={handleClickCell}
      onExitGame={exitGame}
      onVerifyOponentsHit={verifyOponentsHit}
      resetGameState={resetGameState}
    />
  );
}
