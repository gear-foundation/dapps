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
import {
  GameEndModal,
  LobbyTimeFinishedModal,
  LobbyTimer,
  StartGameModal,
  type GameEndData,
} from '@/features/game/components';
import { usePlayerCards, useGameStatus, usePlayerSlots, useTurn, useCountdown } from '@/features/game/hooks';
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
  useCancelGameMessage,
  useCancelRegistrationMessage,
  useEventKilledSubscription,
  useWaitingParticipantsQuery,
  useEventRegisteredToTheNextRoundSubscription,
  useEventWaitingForCardsToBeDisclosedSubscription,
  useEventFinishedSubscription,
  useEventAdminChangedSubscription,
  useAllInPlayersQuery,
  useEventWaitingForAllTableCardsToBeDisclosedSubscription,
  useBlindsQuery,
  useEventLobbyTimeFinishedSubscription,
  useRetiredPlayersQuery,
} from '@/features/game/sails';
import {
  useZkBackend,
  useZkCardDisclosure,
  useZkPartialDecryptionsForPlayersCards,
  useZkTableCardsDecryption,
} from '@/features/zk/hooks';
import { getRankFromValue } from '@/features/zk/utils';

import styles from './game.module.scss';

function GamePage() {
  const navigate = useNavigate();
  const alert = useAlert();
  const { account } = useAccount();

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
    isLobbyTimeFinished,
    isWaitingZk,
    isActiveGame,
    pots,
    refetchStatus,
  } = useGameStatus();

  const { killMessage, isPending: isKillPending } = useKillMessage();
  const { cancelGameMessage, isPending: isCancelGamePending } = useCancelGameMessage();
  const { cancelRegistrationMessage, isPending: isCancelRegistrationPending } = useCancelRegistrationMessage();
  const { retiredPlayers, refetch: refetchRetiredPlayers } = useRetiredPlayersQuery();
  const isRetired = retiredPlayers?.some((address) => address === account?.decodedAddress);

  const [isGameEndModalOpen, setIsGameEndModalOpen] = useState(false);

  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { refetch: refetchAlreadyInvestedInTheCircle } = useAlreadyInvestedInTheCircleQuery();
  const { refetch: refetchAllInPlayers } = useAllInPlayersQuery();
  const { refetch: refetchActiveParticipants } = useActiveParticipantsQuery();
  const { betting, refetch: refetchBetting } = useBettingQuery();
  const { bettingBank, refetch: refetchBettingBank } = useBettingBankQuery();
  const { blinds } = useBlindsQuery();
  const { current_bet } = betting || {};

  const { restartGameMessage, isPending: isRestartGamePending } = useRestartGameMessage();
  const { tableCards, refetch: refetchRevealedTableCards } = useRevealedTableCardsQuery({ enabled: isGameStarted });

  const { playerCards, myCardsC0, refetchPlayerCards } = usePlayerCards(isGameStarted) || {};
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
    void refetchRetiredPlayers();
  };

  useEventRegisteredSubscription({ onData: onPlayersChanged });
  useEventPlayerDeletedSubscription({ onData: onPlayersChanged });
  useEventRegistrationCanceledSubscription({
    onData: ({ player_id }) => {
      const participant = participants?.find(([address]) => address === player_id);
      // TODO: callback acts 2 times on the same event (queryKey), need to fix it
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

  useEventKilledSubscription({
    onData: () => {
      alert.info('Game canceled');
      navigate(ROUTES.HOME);
    },
  });

  useEventDeckShuffleCompleteSubscription({ onData: () => void refetchStatus() });
  useEventGameStartedSubscription({
    onData: () => {
      void refetchStatus();
      setGameEndData(null);
      setIsGameEndModalOpen(false);
    },
  });
  useEventNextStageSubscription({
    onData: (data) => {
      void refetchStatus();
      void refetchAllInPlayers();
      void refetchAlreadyInvestedInTheCircle();
      void refetchActiveParticipants();
      if (data === 'WaitingTableCardsAfterPreFlop') {
        void refetchRevealedTableCards();
      }
    },
  });

  // waitingPartialDecryptionsForPlayersCards -> play.PreFlop
  useEventAllPartialDecryptionsSubmitedSubscription({
    onData: () => {
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
      void refetchAllInPlayers();
    },
  });

  // Get table cards after PreFlop, Flop, Turn
  useEventTablePartialDecryptionsSubmitedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchBetting();
      void refetchRevealedTableCards();
    },
  });

  useEventCardsDisclosedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchRevealedPlayers();
    },
  });

  useEventFinishedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchAllInPlayers();
      void refetchParticipants();
      void refetchActiveParticipants();
      void refetchBetting();
      void refetchBettingBank();
      void refetchAlreadyInvestedInTheCircle();
      void refetchAllInPlayers();
    },
  });

  const onGameRestarted = () => {
    void refetchStatus();
    void refetchBetting();
    void refetchBettingBank();
    void refetchParticipants();
    void refetchActiveParticipants();
    void refetchAlreadyInvestedInTheCircle();
    void refetchAllInPlayers();
    void refetchRevealedPlayers();
    void refetchPlayerCards();
    void refetchRevealedTableCards();
    void refetchWaitingParticipants();
  };

  useEventGameRestartedSubscription({ onData: onGameRestarted });
  useEventGameCanceledSubscription({
    onData: () => {
      alert.info('Game canceled');
      onGameRestarted();
    },
  });

  useEventRegisteredToTheNextRoundSubscription({
    onData: () => {
      void refetchWaitingParticipants();
    },
  });

  const { config, refetch: refetchConfig } = useConfigQuery();
  useEventAdminChangedSubscription({ onData: () => void refetchConfig() });
  useEventLobbyTimeFinishedSubscription({ onData: () => void refetchStatus() });

  const isAdmin = account?.decodedAddress === config?.admin_id;

  useEventWaitingForCardsToBeDisclosedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchActiveParticipants();
    },
  });

  useEventWaitingForAllTableCardsToBeDisclosedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchActiveParticipants();
    },
  });

  const { gameProgress: zkProgress } = useZkBackend({ isWaitingShuffleVerification, isDisabled: isSpectator });

  useZkTableCardsDecryption({
    isWaitingTableCardsAfterPreFlop,
    isWaitingTableCardsAfterFlop,
    isWaitingTableCardsAfterTurn,
    isWaitingForAllTableCardsToBeDisclosed,
    isDisabled: isSpectator,
  });

  useZkPartialDecryptionsForPlayersCards(isWaitingPartialDecryptionsForPlayersCards, isSpectator);
  useZkCardDisclosure(isWaitingForCardsToBeDisclosed, myCardsC0, isSpectator);

  const { onTimeEnd, currentTurn, autoFoldPlayers, timeToTurnEndSec, dillerAddress, isTurnTimeExpired } =
    useTurn(isActiveGame);

  const { remainingMs: lobbyTimeRemainingMs } = useCountdown(config?.lobby_time_limit_ms);

  const playerSlots = usePlayerSlots(currentTurn || null, autoFoldPlayers, playerCards, dillerAddress);

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
      setIsGameEndModalOpen(true);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isFinished, revealedPlayers]);

  useEffect(() => {
    if (!isFinished) return;

    if (isAdmin && !isRestartGamePending) {
      setTimeout(() => {
        void restartGameMessage();
      }, 2000);
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
            onClick={() => (isRegistration ? killMessage() : cancelGameMessage())}
            disabled={isKillPending || isCancelGamePending}
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
          <>
            <GameBoard
              totalPot={gameEndData?.totalPot || totalPot}
              currentBet={current_bet ? Number(current_bet) : undefined}
              showCurrentBet={!isWaitingZk}
              commonCardsFields={gameEndData?.commonCardsFields || commonCardsFields}
              playerSlots={gameEndData?.playerSlots || playerSlots}
              timePerMoveSec={timeToTurnEndSec}
              onTimeEnd={onTimeEnd}
            />
            {lobbyTimeRemainingMs && <LobbyTimer remainingMs={lobbyTimeRemainingMs} />}
          </>
        )}
        {isMyTurn && !isTurnTimeExpired && (
          <GameButtons
            currentBet={Number(current_bet || 0)}
            bigBlind={Number(blinds?.[1] || 0)}
            myCurrentBet={myCurrentBet || 0}
            balance={myBalance}
          />
        )}
        {isMyTurn && timeToTurnEndSec && <YourTurn timePerMoveSec={timeToTurnEndSec} onTimeEnd={onTimeEnd} />}
      </div>

      {!isGameStarted && !isGameEndModalOpen && participants && config && (
        <StartGameModal
          isAdmin={isAdmin}
          participants={startGameParticipants}
          isDefaultExpanded={!gameEndData || isAdmin}
          timeUntilStartMs={config.time_until_start_ms}
          isRetired={isRetired}
        />
      )}

      {gameEndData && isGameEndModalOpen && (
        <GameEndModal {...gameEndData} onClose={() => setIsGameEndModalOpen(false)} isSpectator={isSpectator} />
      )}

      {isLobbyTimeFinished && (
        <LobbyTimeFinishedModal isAdmin={isAdmin} isLoading={isKillPending} onCloseLobby={() => cancelGameMessage()} />
      )}

      {isWaitingZk &&
        (() => {
          const zkVerificationProps = {
            isWaitingShuffleVerification,
            isWaitingPartialDecryptionsForPlayersCards,
            isWaitingTableCards,
            isWaitingForCardsToBeDisclosed,
            isWaitingForAllTableCardsToBeDisclosed,
            zkProgress,
            waitingPlayerName: participants?.find(([address]) => address === zkProgress?.currentPlayerAddress)?.[1]
              .name,
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
