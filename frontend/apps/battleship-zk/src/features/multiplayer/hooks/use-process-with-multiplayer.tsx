import { useAccount } from '@gear-js/react-hooks';
import { useMemo } from 'react';
import { useMoveTransaction, usePending } from '@/features/game/hooks';
import { useMultiplayerGame } from './use-multiplayer-game';
import { useEventGameEndSubscription } from '../sails/events';
import { useCancelGameMessage, useLeaveGameMessage, useMakeMoveMessage } from '../sails/messages';

export const useProcessWithMultiplayer = () => {
  const { game, triggerGame } = useMultiplayerGame();
  const { leaveGameMessage } = useLeaveGameMessage();
  const { cancelGameMessage } = useCancelGameMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const moveTransaction = useMoveTransaction('multi', makeMoveMessage, triggerGame);
  const { account } = useAccount();
  const { gameEndResult } = useEventGameEndSubscription();
  const { setPending } = usePending();

  const participant = game?.participants_data.find((item) => item[0] === account?.decodedAddress)?.[1];

  const totalShoots = useMemo(() => (participant ? participant?.total_shots : 0), [game]);
  const successfulShoots = useMemo(() => (participant ? participant?.succesfull_shots : 0), [game]);

  const gameUpdatedEvent = useMemo(() => {
    // ! TODO: seems like unnecessary, try remove `gameEndResult`
    if (gameEndResult) {
      return { turn: '' };
    }
    const [pendingVerification, verificationRequired] =
      (game && 'pendingVerificationOfTheMove' in game.status && game.status.pendingVerificationOfTheMove) || [];

    return {
      turn: (game && 'turn' in game.status && game.status.turn) || '',
      pendingVerification,
      verificationRequired: pendingVerification === account?.decodedAddress ? verificationRequired : undefined,
    };
  }, [game, gameEndResult]);

  const exitGame = async () => {
    if (game?.admin === account?.decodedAddress) {
      try {
        const transaction = await cancelGameMessage();
        const { response } = await transaction.signAndSend();

        await response();
      } catch (err) {
      } finally {
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
      setPending(false);
    }
  };

  const verifyOponentsHit = () => moveTransaction(null, game?.admin);
  const handleClickCell = (indexCell: number) => moveTransaction(indexCell, game?.admin);

  return {
    totalShoots,
    successfulShoots,
    gameEndResult,
    gameStartTime: game?.start_time || undefined,
    gameUpdatedEvent,
    handleClickCell,
    verifyOponentsHit,
    exitGame,
  };
};
