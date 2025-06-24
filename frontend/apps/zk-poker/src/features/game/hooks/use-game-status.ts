import { useMemo } from 'react';

import { useStatusQuery } from '../sails';

export const useGameStatus = () => {
  const { status } = useStatusQuery();

  return useMemo(() => {
    const isRegistration = status && 'registration' in status;
    const isWaitingShuffleVerification = Boolean(status && 'waitingShuffleVerification' in status);
    const isWaitingPartialDecryptionsForPlayersCards = Boolean(
      status && 'waitingPartialDecryptionsForPlayersCards' in status,
    );
    const isWaitingStart = Boolean(status && 'waitingStart' in status);
    const isPlayStatus = status && 'play' in status;
    const isWaitingTableCardsAfterPreFlop = isPlayStatus && status.play.stage === 'WaitingTableCardsAfterPreFlop';
    const isWaitingTableCardsAfterFlop = isPlayStatus && status.play.stage === 'WaitingTableCardsAfterFlop';
    const isWaitingTableCardsAfterTurn = isPlayStatus && status.play.stage === 'WaitingTableCardsAfterTurn';
    const isWaitingTableCards =
      isWaitingTableCardsAfterPreFlop || isWaitingTableCardsAfterFlop || isWaitingTableCardsAfterTurn;
    const isWaitingForCardsToBeDisclosed = Boolean(status && 'waitingForCardsToBeDisclosed' in status);
    const isWaitingForAllTableCardsToBeDisclosed = Boolean(status && 'waitingForAllTableCardsToBeDisclosed' in status);
    const isGameStarted = !isRegistration && !isWaitingShuffleVerification && !isWaitingStart;
    const isFinished = status && 'finished' in status;
    const isWaitingZk =
      isWaitingShuffleVerification ||
      isWaitingPartialDecryptionsForPlayersCards ||
      isWaitingTableCards ||
      isWaitingForCardsToBeDisclosed ||
      isWaitingForAllTableCardsToBeDisclosed;
    const isActiveGame = isGameStarted && !isFinished && !isWaitingZk;

    const { pots } = (isFinished && status.finished) || {};

    return {
      isRegistration,
      isWaitingShuffleVerification,
      isWaitingPartialDecryptionsForPlayersCards,
      isWaitingStart,
      isPlayStatus,
      isWaitingTableCardsAfterPreFlop,
      isWaitingTableCardsAfterFlop,
      isWaitingTableCardsAfterTurn,
      isWaitingTableCards,
      isWaitingForCardsToBeDisclosed,
      isWaitingForAllTableCardsToBeDisclosed,
      isGameStarted,
      isFinished,
      isWaitingZk,
      isActiveGame,
      pots,
    };
  }, [status]);
};
