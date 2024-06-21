import { useAccount, useApi } from '@gear-js/react-hooks';
import { useMemo } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { ZkProofData } from '@/features/zk/types';
import { usePending } from '@/features/game/hooks';
import { useMultiplayerGame } from './use-multiplayer-game';
import { useEventGameEndSubscription } from '../sails/events';
import { useLeaveGameMessage, useMakeMoveMessage, useVerifyMoveMessage } from '../sails/messages';

export const useProcessWithMultiplayer = () => {
  const { game, triggerGame } = useMultiplayerGame();
  const { leaveGameMessage } = useLeaveGameMessage();
  const { verifyMove } = useVerifyMoveMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const { account } = useAccount();
  const { api } = useApi();
  const { getBoard } = useShips();
  const { getProofData, clearProofData } = useProofShipHit();
  const { gameEndResult } = useEventGameEndSubscription();
  const { setPending } = usePending();

  const enemyBoard = getBoard('multi', 'enemy');

  const totalShoots = useMemo(
    () => (enemyBoard ? enemyBoard.reduce((acc, item) => (item !== 'Unknown' ? acc + 1 : acc), 0) : 0),
    [game],
  );
  const successfulShoots = useMemo(
    () =>
      enemyBoard ? enemyBoard.reduce((acc, item) => (['DeadShip', 'BoomShip'].includes(item) ? acc + 1 : acc), 0) : 0,
    [game],
  );

  const gameUpdatedEvent = useMemo(
    () => ({
      turn: game?.status?.turn || '',
      pendingVerification: game?.status?.pendingVerificationOfTheMove?.[0] || '',
    }),
    [game],
  );

  const exitGame = async () => {
    if (!account?.address) {
      return;
    }

    setPending(true);

    try {
      const transaction = await leaveGameMessage();
      const { response } = await transaction.signAndSend();

      await response();
    } catch (err) {
    } finally {
      await triggerGame();

      setPending(false);
    }
  };

  const getVerifyTransaction = async (proofDataHit: ZkProofData | null | undefined) => {
    if (!proofDataHit) {
      return;
    }
    if (!game) {
      throw new Error('Game now found');
    }

    const { proofContent, publicContent } = proofDataHit;

    const transaction = await verifyMove(
      proofContent,
      {
        hash: publicContent.publicHash,
        out: publicContent.results[0][0],
        hit: publicContent.results[1][0],
      },
      game.admin,
    );

    return transaction;
  };

  const getHitTransaction = async (indexCell: number) => {
    if (!game) {
      throw new Error('Game now found');
    }

    const transaction = await makeMoveMessage(game.admin, indexCell);

    return transaction;
  };

  const handleClickCell = async (indexCell: number) => {
    if (!account?.address || !api) {
      return;
    }

    const proofDataHit = getProofData('multi');

    try {
      const hitTransaction = await getHitTransaction(indexCell);
      const verifyTransaction = await getVerifyTransaction(proofDataHit);

      if (verifyTransaction) {
        const { response } = await verifyTransaction.signAndSend();
        await response();

        clearProofData('multi');
      }

      const { response } = await hitTransaction.signAndSend();
      await response();

      await triggerGame();
    } catch (error) {
      console.log(error);
    }
  };

  return {
    totalShoots,
    successfulShoots,
    gameEndResult,
    gameStartTime: game?.start_time || undefined,
    gameUpdatedEvent,
    handleClickCell,
    exitGame,
  };
};
