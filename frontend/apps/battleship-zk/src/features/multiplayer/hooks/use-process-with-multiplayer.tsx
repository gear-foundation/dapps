import { useAccount, useApi } from '@gear-js/react-hooks';
import { useEffect, useMemo, useState } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { ZkProofData } from '@/features/zk/types';
import { usePending } from '@/features/game/hooks';
import { useMultiplayerGame } from './use-multiplayer-game';
import { useEventGameEndSubscription } from '../sails/events';
import { useCancelGameMessage, useLeaveGameMessage, useMakeMoveMessage, useVerifyMoveMessage } from '../sails/messages';

export const useProcessWithMultiplayer = () => {
  const { game, triggerGame } = useMultiplayerGame();
  const { leaveGameMessage } = useLeaveGameMessage();
  const { cancelGameMessage } = useCancelGameMessage();
  const { verifyMove } = useVerifyMoveMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const { account } = useAccount();
  const { api } = useApi();
  const { getProofData, clearProofData } = useProofShipHit();
  const { gameEndResult } = useEventGameEndSubscription();
  const { setPending } = usePending();

  const participant = game?.participants_data.find((item) => item[0] === account?.decodedAddress)?.[1];

  const totalShoots = useMemo(() => (participant ? participant?.total_shots : 0), [game]);
  const successfulShoots = useMemo(() => (participant ? participant?.succesfull_shots : 0), [game]);

  const gameUpdatedEvent = useMemo(
    () => ({
      turn: game?.status?.turn || '',
      pendingVerification: game?.status?.pendingVerificationOfTheMove?.[0] || '',
    }),
    [game],
  );

  const exitGame = async () => {
    if (game?.admin === account?.decodedAddress) {
      try {
        const transaction = await cancelGameMessage();
        const { response } = await transaction.signAndSend();

        await response();
      } catch (err) {
      } finally {
        await triggerGame();

        setPending(false);
      }

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
      const verifyTransaction = await getVerifyTransaction(proofDataHit);

      if (verifyTransaction) {
        const { response } = await verifyTransaction.signAndSend();
        await response();

        clearProofData('multi');
      }

      const hitTransaction = await getHitTransaction(indexCell);

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
