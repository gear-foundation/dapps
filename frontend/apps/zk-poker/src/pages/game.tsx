import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon, Exit } from '@/assets/images';
import {
  Button,
  CardsLoader,
  Combinations,
  GameBoard,
  GameButtons,
  Header,
  OperationLogs,
  YourTurn,
  ZkVerification,
} from '@/components';
import { GameEndModal, StartGameModal, type GameEndData } from '@/features/game/components';
import { usePlayerCards, useGameStatus, usePlayerSlots, useTurn } from '@/features/game/hooks';
import {
  useParticipantsQuery,
  useEventRegisteredSubscription,
  useConfigQuery,
  useEventPlayerDeletedSubscription,
  useBettingQuery,
  useBettingBankQuery,
  useRestartGameMessage,
  useEventNextStageSubscription,
  useEventTurnIsMadeSubscription,
  useRevealedTableCardsQuery,
  useEventDeckShuffleCompleteSubscription,
  useEventGameStartedSubscription,
  useEventGameRestartedSubscription,
  useEventRegistrationCanceledSubscription,
  useActiveParticipantsQuery,
  useAlreadyInvestedInTheCircleQuery,
  useEventTablePartialDecryptionsSubmitedSubscription,
  useEventCardsDisclosedSubscription,
  useEventGameCanceledSubscription,
  useRevealedPlayersQuery,
  useEventAllPartialDecryptionsSubmitedSubscription,
  useKillMessage,
  useCancelRegistrationMessage,
  useEventKilledSubscription,
  useWaitingParticipantsQuery,
  useEventRegisteredToTheNextRoundSubscription,
  useEventWaitingForCardsToBeDisclosedSubscription,
  useEventFinishedSubscription,
} from '@/features/game/sails';
import { useZkBackend, useZkCardDisclosure, useZkTableCardsDecryption } from '@/features/zk/hooks';
import { getRankFromValue } from '@/features/zk/utils';

import styles from './game.module.scss';

function GamePage() {
  const navigate = useNavigate();
  const alert = useAlert();

  const [gameEndData, setGameEndData] = useState<GameEndData | null>(null);

  const {
    isRegistration,
    isWaitingShuffleVerification,
    isWaitingPartialDecryptionsForPlayersCards,
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
    refetchStatus,
  } = useGameStatus();

  const { killMessage, isPending: isKillPending } = useKillMessage();
  const { cancelRegistrationMessage, isPending: isCancelRegistrationPending } = useCancelRegistrationMessage();

  const { account } = useAccount();
  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { refetch: refetchAlreadyInvestedInTheCircle } = useAlreadyInvestedInTheCircleQuery();
  const { refetch: refetchActiveParticipants } = useActiveParticipantsQuery();
  const { betting, refetch: refetchBetting } = useBettingQuery();
  const { bettingBank, refetch: refetchBettingBank } = useBettingBankQuery();
  const { current_bet } = betting || {};

  const { restartGameMessage, isPending: isRestartGamePending } = useRestartGameMessage();
  const { tableCards, refetch: refetchRevealedTableCards } = useRevealedTableCardsQuery({ enabled: isGameStarted });

  const { playerCards, inputs, refetchPlayerCards } = usePlayerCards(isGameStarted) || {};
  const { revealedPlayers, refetch: refetchRevealedPlayers } = useRevealedPlayersQuery({
    enabled: isFinished,
  });
  const { waitingParticipants, refetch: refetchWaitingParticipants } = useWaitingParticipantsQuery();
  const isSpectator = !participants?.some(([address]) => address === account?.decodedAddress);
  const startGameParticipants = participants && waitingParticipants ? [...participants, ...waitingParticipants] : [];

  const onPlayersChanged = () => {
    void refetchStatus();
    void refetchParticipants();
    void refetchActiveParticipants();
  };

  useEventRegisteredSubscription({ onData: onPlayersChanged });
  useEventPlayerDeletedSubscription({ onData: onPlayersChanged });
  useEventRegistrationCanceledSubscription({
    onData: ({ player_id }) => {
      const participant = participants?.find(([address]) => address === player_id);
      // ! TODO: callback acts 2 times on the same event (queryKey), need to fix it
      if (participant) {
        const message =
          account?.decodedAddress === participant?.[0]
            ? 'You left the game'
            : `Player ${participant?.[1].name} left the game`;
        alert.info(message);
      }
      onPlayersChanged();
    },
    queryKey: [participants, account?.decodedAddress],
  });
  useEventGameCanceledSubscription({
    onData: () => {
      alert.info('Game restarted');
    },
  });

  useEventKilledSubscription({
    onData: () => {
      alert.info('Game canceled');
      navigate(ROUTES.HOME);
    },
  });

  useEventDeckShuffleCompleteSubscription({ onData: () => void refetchStatus() });
  useEventGameStartedSubscription({ onData: () => void refetchStatus() });
  useEventNextStageSubscription({
    onData: (data) => {
      void refetchStatus();
      if (data === 'WaitingTableCardsAfterPreFlop') {
        void refetchRevealedTableCards();
      }
    },
  });

  // waitingPartialDecryptionsForPlayersCards -> play.PreFlop
  useEventAllPartialDecryptionsSubmitedSubscription({
    onData: () => {
      console.log('!!!!! ~ useEventAllPartialDecryptionsSubmitedSubscription');
      void refetchStatus();
      void refetchPlayerCards();
      void refetchBetting();
      void refetchBettingBank();
      void refetchAlreadyInvestedInTheCircle();
    },
  });

  useEventTurnIsMadeSubscription({
    onData: () => {
      void refetchStatus();
      void refetchParticipants();
      void refetchActiveParticipants();
      void refetchBetting();
      void refetchBettingBank();
      void refetchAlreadyInvestedInTheCircle();
    },
  });

  // Get table cards after PreFlop, Flop, Turn
  useEventTablePartialDecryptionsSubmitedSubscription({
    onData: () => {
      console.log('!!!!! ~ useEventTablePartialDecryptionsSubmitedSubscription');
      void refetchStatus();
      void refetchBetting();
      void refetchRevealedTableCards();
    },
  });

  useEventCardsDisclosedSubscription({
    onData: () => {
      console.log('!!!!! ~ useEventCardsDisclosedSubscription');
      void refetchStatus();
      void refetchRevealedPlayers();
    },
  });

  useEventFinishedSubscription({
    onData: () => {
      console.log('!!!!! ~ useEventFinishedSubscription');
      void refetchStatus();
      void refetchParticipants();
      void refetchBetting();
      void refetchBettingBank();
      void refetchAlreadyInvestedInTheCircle();
    },
  });

  useEventGameRestartedSubscription({
    onData: () => {
      console.log('!!!!! ~ useEventGameRestartedSubscription');
      void refetchStatus();
      void refetchBetting();
      void refetchBettingBank();
      void refetchActiveParticipants();
      void refetchAlreadyInvestedInTheCircle();
      void refetchRevealedPlayers();
      void refetchPlayerCards();
      void refetchRevealedTableCards();
      void refetchWaitingParticipants();
    },
  });

  useEventRegisteredToTheNextRoundSubscription({
    onData: () => {
      void refetchWaitingParticipants();
    },
  });

  const { config } = useConfigQuery();

  const isAdmin = account?.decodedAddress === config?.admin_id;

  useEventWaitingForCardsToBeDisclosedSubscription({
    onData: () => {
      console.log('!!!! ~ waiting for cards to be disclosed');
      void refetchStatus();
    },
  });
  useZkBackend({
    isWaitingShuffleVerification,
    isWaitingPartialDecryptionsForPlayersCards,
    isDisabled: isSpectator,
  });

  useZkTableCardsDecryption({
    isWaitingTableCardsAfterPreFlop,
    isWaitingTableCardsAfterFlop,
    isWaitingTableCardsAfterTurn,
    isWaitingForAllTableCardsToBeDisclosed,
    isDisabled: isSpectator,
  });

  useZkCardDisclosure(isWaitingForCardsToBeDisclosed, inputs, playerCards, isSpectator);

  const { onTimeEnd, currentTurn, autoFoldPlayers, timeToTurnEndSec, dillerAddress } = useTurn();
  const playerSlots = usePlayerSlots(currentTurn || null, autoFoldPlayers, dillerAddress);

  const commonCardsFields = [null, null, null, null, null].map((_, index) => {
    const card = tableCards?.[index];
    if (card) {
      const { suit, value } = card;
      return { suit, rank: getRankFromValue(value) };
    }
    return null;
  });

  const totalPot = bettingBank?.reduce((acc, [, amount]) => acc + Number(amount), 0) || undefined;

  const isMyTurn = currentTurn === account?.decodedAddress && isActiveGame;
  const myCurrentBet = playerSlots?.find(({ isMe }) => isMe)?.bet;
  const myBalance = Number(participants?.find(([address]) => address === account?.decodedAddress)?.[1].balance || 0);

  useEffect(() => {
    if (!isFinished) return;

    // Save game end data when game finishes
    if (participants && pots && revealedPlayers && commonCardsFields && totalPot) {
      setGameEndData({
        pots,
        revealedPlayers,
        commonCardsFields,
        participants,
        playerSlots,
        totalPot,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isFinished, revealedPlayers]);

  useEffect(() => {
    if (!isFinished) return;

    if (isAdmin && !isRestartGamePending) {
      setTimeout(() => {
        // ! TODO: refetch on error
        void restartGameMessage();
      }, 3000);
    }
  }, [isFinished, restartGameMessage, isAdmin, isRestartGamePending]);

  return (
    <>
      <Header>
        {isAdmin ? (
          <Button
            color="danger"
            rounded
            size="medium"
            onClick={() => killMessage()}
            disabled={isKillPending}
            className={styles.killButton}>
            <Exit />
          </Button>
        ) : (
          <Button
            color="contrast"
            rounded
            size="medium"
            onClick={() => (isSpectator ? navigate(ROUTES.HOME) : cancelRegistrationMessage())}
            disabled={isCancelRegistrationPending}
            className={styles.backButton}>
            {isSpectator ? <BackIcon /> : <Exit />}
          </Button>
        )}
        <Combinations />
      </Header>

      {isMyTurn && <div className={styles.bottomGlow} />}
      <div className={styles.content}>
        {config && (
          <GameBoard
            totalPot={gameEndData?.totalPot || totalPot}
            currentBet={current_bet ? Number(current_bet) : undefined}
            showCurrentBet={!isWaitingZk}
            commonCardsFields={gameEndData?.commonCardsFields || commonCardsFields}
            playerSlots={gameEndData?.playerSlots || playerSlots}
            timePerMoveSec={timeToTurnEndSec}
            onTimeEnd={onTimeEnd}
          />
        )}
        {isMyTurn && (
          <GameButtons
            currentBet={Number(current_bet || 0)}
            bigBlind={Number(config?.big_blind || 0)}
            myCurrentBet={myCurrentBet || 0}
            balance={myBalance}
          />
        )}
        {isMyTurn && timeToTurnEndSec && <YourTurn timePerMoveSec={timeToTurnEndSec} onTimeEnd={onTimeEnd} />}
      </div>

      {!isGameStarted && !gameEndData && participants && config && (
        <StartGameModal isAdmin={isAdmin} participants={startGameParticipants} />
      )}

      {gameEndData && <GameEndModal {...gameEndData} onClose={() => setGameEndData(null)} />}

      {isWaitingZk &&
        (() => {
          const zkVerificationProps = {
            isWaitingShuffleVerification,
            isWaitingPartialDecryptionsForPlayersCards,
            isWaitingTableCards,
            isWaitingForCardsToBeDisclosed,
            isWaitingForAllTableCardsToBeDisclosed,
          };

          const showInLoader = isWaitingShuffleVerification || isWaitingPartialDecryptionsForPlayersCards;

          return showInLoader ? (
            <CardsLoader>
              <ZkVerification {...zkVerificationProps} isInLoader />
            </CardsLoader>
          ) : (
            <ZkVerification {...zkVerificationProps} />
          );
        })()}

      <OperationLogs isHidden={isRegistration || isMyTurn} />
    </>
  );
}

export default GamePage;
