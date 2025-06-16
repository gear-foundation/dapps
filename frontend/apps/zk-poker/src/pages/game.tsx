import { HexString } from '@gear-js/api';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon, Exit } from '@/assets/images';
import { Button, CardsLoader, GameBoard, GameButtons, Header, YourTurn, ZkVerification } from '@/components';
import { GameEndModal, StartGameModal } from '@/features/game/components';
import { usePlayerCards } from '@/features/game/hooks';
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
import { Card, PlayerStatus } from '@/features/zk/api/types';
import { useZkBackend, useZkCardDisclosure, useZkTableCardsDecryption } from '@/features/zk/hooks';
import { getRankFromValue } from '@/features/zk/utils';

import styles from './game.module.scss';

export default function GamePage() {
  const navigate = useNavigate();
  const { status, refetch: refetchStatus } = useStatusQuery();
  console.log('🚀 ~ GamePage ~ status:', status);
  const alert = useAlert();

  // ! TODO: move to separate file useStatus
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

  const { killMessage, isPending: isKillPending } = useKillMessage();
  const { cancelRegistrationMessage, isPending: isCancelRegistrationPending } = useCancelRegistrationMessage();

  const { account } = useAccount();
  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { alreadyInvestedInTheCircle, refetch: refetchAlreadyInvestedInTheCircle } =
    useAlreadyInvestedInTheCircleQuery();
  const { activeParticipants, refetch: refetchActiveParticipants } = useActiveParticipantsQuery();
  const { betting, refetch: refetchBetting } = useBettingQuery();
  const { bettingBank, refetch: refetchBettingBank } = useBettingBankQuery();
  const { turn, current_bet } = betting || {};
  const { cash_prize, winners } = (isFinished && status.finished) || {};

  const { restartGameMessage, isPending: isRestartGamePending } = useRestartGameMessage();
  const { tableCards, refetch: refetchRevealedTableCards } = useRevealedTableCardsQuery({ enabled: isGameStarted });

  const { playerCards, instances, refetchPlayerCards } = usePlayerCards(isGameStarted) || {};
  const { revealedPlayers, refetch: refetchRevealedPlayers } = useRevealedPlayersQuery({
    enabled: isFinished,
  });

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
  });

  useZkTableCardsDecryption({
    isWaitingTableCardsAfterPreFlop,
    isWaitingTableCardsAfterFlop,
    isWaitingTableCardsAfterTurn,
    isWaitingForAllTableCardsToBeDisclosed,
    onEvent: () => {
      void refetchStatus();
    },
  });
  useZkCardDisclosure(isWaitingForCardsToBeDisclosed, instances);

  const getPlayerCards = (address: string) => {
    if (address === account?.decodedAddress && playerCards) {
      return playerCards as [Card, Card];
    }

    const revealedPlayer = revealedPlayers?.find(([playerAddress]) => playerAddress === address);
    if (revealedPlayer) {
      return revealedPlayer[1].map((card) => ({
        suit: card.suit,
        rank: getRankFromValue(card.value),
      })) as [Card, Card];
    }

    return playerCards ? null : undefined;
  };

  const getStatusAndBet = (address: HexString): { status: PlayerStatus; bet?: number } => {
    const investedInTheCircle = alreadyInvestedInTheCircle?.find(([actorId]) => actorId === address);

    if (winners?.includes(address)) {
      return { status: 'winner' };
    }

    const isHaveNoBalance = participants?.find(([actorId]) => actorId === address)?.[1].balance === 0;
    const isHaveBet = bettingBank?.find(([actorId]) => actorId === address)?.[1] !== 0;
    if (isHaveNoBalance && isHaveBet) {
      return { status: 'all-in' };
    }

    if (!activeParticipants?.active_ids?.includes(address)) {
      return { status: 'fold' };
    }

    if (address === turn && turn !== account?.decodedAddress && isActiveGame) {
      return { status: 'thinking' };
    }

    const isActed = betting?.acted_players?.find((actorId) => actorId === address);
    if (isActed && !investedInTheCircle) {
      return { status: 'check' };
    }

    if (investedInTheCircle) {
      const [, amount] = investedInTheCircle;
      return { status: 'bet', bet: Number(amount) };
    }
    return { status: 'waiting' };
  };

  const playerSlots =
    participants?.map(([address, participant]) => ({
      address,
      name: participant.name,
      chips: Number(participant.balance),
      cards: getPlayerCards(address),
      isMe: address === account?.decodedAddress,
      ...getStatusAndBet(address),
      // ! TODO: derive diller
      // isDiller: true,
    })) || [];

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
            onClick={() => cancelRegistrationMessage().then(() => navigate(ROUTES.HOME))}
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
          />
        )}
        {isMyTurn && config && <YourTurn timePerMoveMs={Number(config.time_per_move_ms)} />}
      </div>

      {!isGameStarted && participants && config && (
        <StartGameModal
          isAdmin={isAdmin}
          participants={participants}
          maxPlayers={config.number_of_participants}
          isWaitingStart={isWaitingStart}
        />
      )}

      {isFinished && participants && (
        <GameEndModal
          cashPrize={cash_prize}
          winners={winners}
          revealedPlayers={revealedPlayers}
          commonCardsFields={commonCardsFields}
          participants={participants}
        />
      )}

      {/* {isWaitingZk && (
        <ZkVerification
          isWaitingShuffleVerification={isWaitingShuffleVerification}
          isWaitingPartialDecryptionsForPlayersCards={isWaitingPartialDecryptionsForPlayersCards}
          isWaitingTableCards={isWaitingTableCards}
          isWaitingForCardsToBeDisclosed={isWaitingForCardsToBeDisclosed}
          isWaitingForAllTableCardsToBeDisclosed={isWaitingForAllTableCardsToBeDisclosed}
        />
      )} */}

      {isWaitingZk && (
        <CardsLoader>
          <ZkVerification
            isWaitingShuffleVerification={isWaitingShuffleVerification}
            isWaitingPartialDecryptionsForPlayersCards={isWaitingPartialDecryptionsForPlayersCards}
            isWaitingTableCards={isWaitingTableCards}
            isWaitingForCardsToBeDisclosed={isWaitingForCardsToBeDisclosed}
            isWaitingForAllTableCardsToBeDisclosed={isWaitingForAllTableCardsToBeDisclosed}
            isInLoader
          />
        </CardsLoader>
      )}
    </>
  );
}
