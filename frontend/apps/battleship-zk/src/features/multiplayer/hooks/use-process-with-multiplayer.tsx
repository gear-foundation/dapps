import { useAccount } from '@gear-js/react-hooks';
import { useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useMoveTransaction, usePending } from '@/features/game/hooks';
import { useMultiplayerGame } from './use-multiplayer-game';
import { useEventGameEndSubscription } from '../sails/events';
import { useCancelGameMessage, useLeaveGameMessage, useMakeMoveMessage } from '../sails/messages';
import { ROUTES } from '@/app/consts';
import { clearZkData } from '@/features/zk/utils';
import { useRemainingTimeQuery } from '../sails/queries/use-remaining-time-query';

export const useProcessWithMultiplayer = () => {
  const { game, triggerGame, resetGameState } = useMultiplayerGame();
  const { leaveGameMessage } = useLeaveGameMessage();
  const navigate = useNavigate();
  const { cancelGameMessage } = useCancelGameMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const moveTransaction = useMoveTransaction('multi', makeMoveMessage, triggerGame);
  const { account } = useAccount();
  const { gameEndResult } = useEventGameEndSubscription();
  const { setPending } = usePending();

  const participantsData = gameEndResult?.participants_info || game?.participants_data;
  const participant = participantsData?.find((item) => item[0] === account?.decodedAddress)?.[1];

  const totalShoots = useMemo(() => (participant ? participant?.total_shots : 0), [game]);
  const successfulShoots = useMemo(() => (participant ? participant?.succesfull_shots : 0), [game]);

  const gameUpdatedEvent = useMemo(() => {
    const [pendingVerification, verificationRequired] =
      (game && 'pendingVerificationOfTheMove' in game.status && game.status.pendingVerificationOfTheMove) || [];

    return {
      turn: (game && 'turn' in game.status && game.status.turn) || '',
      pendingVerification,
      verificationRequired: pendingVerification === account?.decodedAddress ? verificationRequired : undefined,
    };
  }, [game]);

  const remainingTime = useRemainingTimeQuery();

  const exitGame = async () => {
    if (gameEndResult) {
      navigate(ROUTES.HOME);
      resetGameState();

      if (account?.address) {
        clearZkData('multi', account);
      }

      return;
    }

    setPending(true);

    try {
      const getTransaction = game?.admin === account?.decodedAddress ? cancelGameMessage : leaveGameMessage;
      const transaction = await getTransaction();
      const { response } = await transaction.signAndSend();

      await response();
    } catch (err) {
    } finally {
      setPending(false);
    }
  };

  const verifyOponentsHit = () => moveTransaction(null, game?.admin);
  const handleClickCell = (indexCell: number) => moveTransaction(indexCell, game?.admin);

  return {
    totalShoots,
    successfulShoots,
    gameEndResult,
    remainingTime,
    gameUpdatedEvent,
    handleClickCell,
    verifyOponentsHit,
    exitGame,
  };
};
