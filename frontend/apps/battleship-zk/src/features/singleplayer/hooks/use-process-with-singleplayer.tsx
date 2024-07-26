import { useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useEventGameEndSubscription } from '../sails/events';
import { useMakeMoveMessage } from '../sails/messages';
import { useSingleplayerGame } from './use-singleplayer-game';
import { useMoveTransaction } from '@/features/game/hooks';
import { ROUTES } from '@/app/consts';

export const useProcessWithSingleplayer = () => {
  const navigate = useNavigate();
  const { makeMoveMessage } = useMakeMoveMessage();
  const { game, triggerGame } = useSingleplayerGame();

  const moveTransaction = useMoveTransaction('single', makeMoveMessage, triggerGame);

  const { gameEndResult } = useEventGameEndSubscription();

  const totalShoots = useMemo(() => (game ? game.total_shots : gameEndResult?.total_shots || 0), [game]);
  const successfulShoots = useMemo(() => (game ? game.succesfull_shots : gameEndResult?.succesfull_shots || 0), [game]);

  const gameUpdatedEvent = useMemo(() => ({ turn: '', verificationRequired: game?.verification_requirement }), [game]);

  const exitGame = async () => {
    navigate(ROUTES.HOME);
  };

  const verifyOponentsHit = () => moveTransaction(null);
  const handleClickCell = (indexCell: number) => moveTransaction(indexCell);

  return {
    totalShoots,
    successfulShoots,
    gameEndResult,
    gameStartTime: game?.start_time,
    gameUpdatedEvent,
    handleClickCell,
    verifyOponentsHit,
    exitGame,
  };
};
