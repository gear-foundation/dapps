import { useAccount, useApi } from '@gear-js/react-hooks';
import { useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useEventGameEndSubscription } from '../sails/events';
import { useMakeMoveMessage, useVerifyMoveMessage } from '../sails/messages';
import { useSingleplayerGame } from './use-singleplayer-game';

export const useProcessWithSingleplayer = () => {
  const navigate = useNavigate();
  const { verifyMoveMessage } = useVerifyMoveMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const { game, triggerGame } = useSingleplayerGame();
  const { account } = useAccount();
  const { api } = useApi();
  const { getProofData, clearProofData } = useProofShipHit();

  const { gameEndResult } = useEventGameEndSubscription();

  const totalShoots = useMemo(() => (game ? game.total_shots : gameEndResult?.total_shots || 0), [game]);
  const successfulShoots = useMemo(() => (game ? game.succesfull_shots : gameEndResult?.succesfull_shots || 0), [game]);

  const gameUpdatedEvent = useMemo(() => ({ turn: '', pendingVerification: '' }), [game]);

  const exitGame = async () => {
    navigate('/');
  };

  const getVerifyTransaction = async (proofDataHit: any) => {
    if (!proofDataHit) {
      return null;
    }

    const { proofContent, publicContent } = proofDataHit;

    const transaction = await verifyMoveMessage(proofContent, {
      hash: publicContent.publicHash,
      out: publicContent.results[0][0],
      hit: publicContent.results[1][0],
    });

    return transaction;
  };

  const getHitTransaction = async (indexCell: number) => {
    const transaction = await makeMoveMessage(indexCell);

    return transaction;
  };

  const handleClickCell = async (indexCell: number) => {
    if (!account?.address || !api) {
      return;
    }

    const proofDataHit = getProofData('single');

    try {
      const verifyTransaction = await getVerifyTransaction(proofDataHit);

      if (verifyTransaction) {
        const { response } = await verifyTransaction.signAndSend();
        await response();

        clearProofData('single');
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
    gameStartTime: game?.start_time,
    gameUpdatedEvent,
    handleClickCell,
    exitGame,
  };
};
