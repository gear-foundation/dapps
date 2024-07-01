import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { GameProcess, ShipArrangement } from '@/features/game';
import { Modal } from '@/components';
import { Button } from '@gear-js/vara-ui';
import { useShips } from '@/features/zk/hooks/use-ships';
import { decodeAddress } from '@gear-js/api';
import { usePending } from '@/features/game/hooks';
import {
  useEventPlacementVerified,
  useEventGameCancelled,
  useEventMoveMadeSubscription,
  useEventMoveVerifiedSubscription,
} from '../../sails/events';
import { useMultiplayerGame, useProcessWithMultiplayer, useArrangementWithMultiplayer } from '../../hooks';
import styles from './Multiplayer.module.scss';

export function Multiplayer() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { gameType, makeStartGameTransaction } = useArrangementWithMultiplayer();
  const { totalShoots, successfulShoots, gameEndResult, gameStartTime, gameUpdatedEvent, handleClickCell, exitGame } =
    useProcessWithMultiplayer();
  const { game, triggerGame } = useMultiplayerGame();
  const { getBoard } = useShips();
  const { pending } = usePending();
  const [savedPlayerBoard, setSavedPlayerBoard] = useState<string[] | null | undefined>();

  useEventPlacementVerified();
  useEventGameCancelled();
  useEventMoveMadeSubscription();
  useEventMoveVerifiedSubscription();

  const playerInfo = game?.participants_data.find((item) => decodeAddress(item[0]) === account?.decodedAddress)?.[1];
  const isPlacementStatus = Object.keys(game?.status || {})[0] === 'verificationPlacement';

  const handleCloseModal = () => {
    navigate('/');
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
    </>
  ) : (
    <GameProcess
      gameType={gameType}
      totalShoots={totalShoots}
      successfulShoots={successfulShoots}
      gameResults={gameEndResult ? { totalTime: gameEndResult.total_time, winner: gameEndResult.winner } : null}
      gameStartTime={gameStartTime}
      gameUpdatedEvent={gameUpdatedEvent}
      admin={game?.admin}
      onClickCell={handleClickCell}
      onExitGame={exitGame}
    />
  );
}
