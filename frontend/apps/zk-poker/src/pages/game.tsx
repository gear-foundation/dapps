import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon, Exit } from '@/assets/images';
import {
  Button,
  CardsLoader,
  GameBoard,
  GameButtons,
  Header,
  OperationLogs,
  YourTurn,
  ZkVerification,
} from '@/components';
import { GameEndModal, StartGameModal } from '@/features/game/components';
import { usePlayerCards, useGameStatus, usePlayerSlots } from '@/features/game/hooks';
import {
  useStatusQuery,
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
} from '@/features/game/sails';
import { useEventFinishedSubscription } from '@/features/game/sails/poker/events/use-event-finished-subscription';
import { useZkBackend, useZkCardDisclosure, useZkTableCardsDecryption } from '@/features/zk/hooks';
import { getRankFromValue } from '@/features/zk/utils';

import styles from './game.module.scss';

export default function GamePage() {
  const navigate = useNavigate();
  const alert = useAlert();
  const { refetch: refetchStatus } = useStatusQuery();

  const {
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
  } = useGameStatus();

  const { killMessage, isPending: isKillPending } = useKillMessage();
  const { cancelRegistrationMessage, isPending: isCancelRegistrationPending } = useCancelRegistrationMessage();

  const { account } = useAccount();
  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { refetch: refetchAlreadyInvestedInTheCircle } = useAlreadyInvestedInTheCircleQuery();
  const { refetch: refetchActiveParticipants } = useActiveParticipantsQuery();
  const { betting, refetch: refetchBetting } = useBettingQuery();
  const { bettingBank, refetch: refetchBettingBank } = useBettingBankQuery();
  const { turn, current_bet } = betting || {};

  const { restartGameMessage, isPending: isRestartGamePending } = useRestartGameMessage();
  const { tableCards, refetch: refetchRevealedTableCards } = useRevealedTableCardsQuery({ enabled: isGameStarted });

  const { playerCards, inputs, refetchPlayerCards } = usePlayerCards(isGameStarted) || {};
  const { revealedPlayers, refetch: refetchRevealedPlayers } = useRevealedPlayersQuery({
    enabled: isFinished,
  });

  const isSpectator = !participants?.some(([address]) => address === account?.decodedAddress);

  const onPlayersChanged = () => {
    void refetchStatus();
    void refetchParticipants();
    void refetchActiveParticipants();
  };

  useEventRegisteredSubscription({ onData: onPlayersChanged });
  useEventPlayerDeletedSubscription({ onData: onPlayersChanged });
  useEventRegistrationCanceledSubscription({
    onData: ({ player_id }) => {
      onPlayersChanged();
      if (player_id === account?.decodedAddress) {
        navigate(ROUTES.HOME);
      }
    },
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
    },
  });

  const { config } = useConfigQuery();

  const isAdmin = account?.decodedAddress === config?.admin_id;

  // useEventWaitingForCardsToBeDisclosedSubscription({
  //   onData: () => {
  //     console.log('!!!! ~ waiting for cards to be disclosed');
  //     void refetchStatus();
  //   },
  // });
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
    onEvent: () => {
      void refetchStatus();
    },
  });
  useZkCardDisclosure(isWaitingForCardsToBeDisclosed, inputs, playerCards, isSpectator);

  const playerSlots = usePlayerSlots();

  const commonCardsFields = [null, null, null, null, null].map((_, index) => {
    const card = tableCards?.[index];
    if (card) {
      const { suit, value } = card;
      return { suit, rank: getRankFromValue(value) };
    }
    return null;
  });

  const totalPot = bettingBank?.reduce((acc, [, amount]) => acc + Number(amount), 0) || undefined;

  const isMyTurn = turn === account?.decodedAddress && isActiveGame;
  const myCurrentBet = playerSlots?.find(({ isMe }) => isMe)?.bet;
  const myBalance = Number(participants?.find(([address]) => address === account?.decodedAddress)?.[1].balance || 0);

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
            onClick={() => killMessage().then(() => navigate(ROUTES.HOME))}
            disabled={isKillPending}
            className={styles.killButton}>
            <Exit />
          </Button>
        ) : (
          <Button
            color="contrast"
            rounded
            size="medium"
            onClick={() =>
              isSpectator ? navigate(ROUTES.HOME) : cancelRegistrationMessage().then(() => navigate(ROUTES.HOME))
            }
            disabled={isCancelRegistrationPending}>
            <BackIcon />
          </Button>
        )}
      </Header>

      {isMyTurn && <div className={styles.bottomGlow} />}
      <div className={styles.content}>
        {config && (
          <GameBoard
            totalPot={totalPot}
            commonCardsFields={commonCardsFields}
            playerSlots={playerSlots}
            timePerMoveMs={Number(config.time_per_move_ms)}
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
        {isMyTurn && config && <YourTurn timePerMoveMs={Number(config.time_per_move_ms)} />}
      </div>

      {!isGameStarted && participants && config && <StartGameModal isAdmin={isAdmin} participants={participants} />}

      {isFinished && participants && (
        <GameEndModal
          pots={pots}
          revealedPlayers={revealedPlayers}
          commonCardsFields={commonCardsFields}
          participants={participants}
        />
      )}

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

      <OperationLogs />
    </>
  );
}
