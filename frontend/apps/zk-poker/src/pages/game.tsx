import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, GameBoard, GameButtons, Header, YourTurn, ZkVerification } from '@/components';
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
  useEventCardsDealtToPlayersSubscription,
  useEventGameRestartedSubscription,
  useEventRegistrationCanceledSubscription,
  useEventCardsDealtToTableSubscription,
  useActiveParticipantsQuery,
  useAlreadyInvestedInTheCircleQuery,
} from '@/features/game/sails';
import { useEventFinishedSubscription } from '@/features/game/sails/poker/events/use-event-finished-subscription';
import { Card, PlayerStatus } from '@/features/zk/api/types';
import { useZkBackend, useZkCardDisclosure, useZkTableCardsDecryption } from '@/features/zk/hooks';
import { getRankFromValue } from '@/features/zk/utils';

import styles from './game.module.scss';

export default function GamePage() {
  const navigate = useNavigate();
  const { status, refetch: refetchStatus } = useStatusQuery();

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
  const isGameStarted = !isRegistration && !isWaitingShuffleVerification && !isWaitingStart;
  const isFinished = status && 'finished' in status;
  const isWaitingZk = isWaitingShuffleVerification || isWaitingPartialDecryptionsForPlayersCards || isWaitingTableCards;
  const isActiveGame = isGameStarted && !isFinished && !isWaitingZk;

  const { account } = useAccount();
  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { alreadyInvestedInTheCircle, refetch: refetchAlreadyInvestedInTheCircle } =
    useAlreadyInvestedInTheCircleQuery();
  // ! TODO: understand if we need this query
  const { activeParticipants, refetch: refetchActiveParticipants } = useActiveParticipantsQuery();
  const { betting, refetch: refetchBetting } = useBettingQuery();
  const { bettingBank, refetch: refetchBettingBank } = useBettingBankQuery();
  const { turn, current_bet } = betting || {};
  const { cash_prize, winners } = (isFinished && status.finished) || {};

  const { restartGameMessage, isPending: isRestartGamePending } = useRestartGameMessage();
  const { tableCards, refetch: refetchTableCards } = useRevealedTableCardsQuery({ enabled: isGameStarted });

  const onPlayersChanged = () => {
    void refetchStatus();
    void refetchParticipants();
  };

  useEventRegisteredSubscription({ onData: onPlayersChanged });
  useEventPlayerDeletedSubscription({ onData: onPlayersChanged });
  useEventRegistrationCanceledSubscription({ onData: onPlayersChanged });

  useEventDeckShuffleCompleteSubscription({ onData: () => void refetchStatus() });
  useEventGameStartedSubscription({ onData: () => void refetchStatus() });
  useEventNextStageSubscription({
    onData: (data) => {
      void refetchStatus();
      if (data === 'WaitingTableCardsAfterPreFlop') {
        void refetchTableCards();
      }
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

  useEventCardsDealtToPlayersSubscription({
    onData: () => {
      void refetchStatus();
      void refetchBetting();
      void refetchBettingBank();
    },
  });
  useEventGameRestartedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchBetting();
      void refetchBettingBank();
      void refetchActiveParticipants();
      void refetchAlreadyInvestedInTheCircle();
    },
  });

  useEventCardsDealtToTableSubscription({
    onData: () => {
      void refetchStatus();
      void refetchTableCards();
    },
  });

  useEventFinishedSubscription({
    onData: () => {
      void refetchStatus();
      void refetchParticipants();
      void refetchBetting();
      void refetchBettingBank();
      void refetchAlreadyInvestedInTheCircle();
    },
  });

  const { config } = useConfigQuery();

  const isAdmin = account?.decodedAddress === config?.admin_id;

  const { cards: playerCards, instances } = usePlayerCards(isGameStarted) || {};

  useZkBackend({
    isWaitingShuffleVerification,
    isWaitingPartialDecryptionsForPlayersCards,
    isWaitingForCardsToBeDisclosed,
  });
  useZkTableCardsDecryption({
    isWaitingTableCardsAfterPreFlop,
    isWaitingTableCardsAfterFlop,
    isWaitingTableCardsAfterTurn,
  });
  useZkCardDisclosure(isWaitingForCardsToBeDisclosed, instances);

  const getPlayerCards = (address: string, _participant: Participant) => {
    if (address === account?.decodedAddress && playerCards) {
      return playerCards as [Card, Card];
    }

    // ! TODO:
    // if (participant.card_1 && participant.card_2) {
    //   return [playerCards?.[0], playerCards?.[1]];
    // }

    return playerCards ? null : undefined;
  };

  const getStatusAndBet = (address: HexString): { status: PlayerStatus; bet?: number } => {
    const investedInTheCircle = alreadyInvestedInTheCircle?.find(([actorId]) => actorId === address);

    if (winners?.includes(address)) {
      return { status: 'winner' };
    }

    if (!activeParticipants?.active_ids?.includes(address)) {
      return { status: 'fold' };
    }

    if (address === turn && isActiveGame) {
      return { status: 'thinking' };
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
      cards: getPlayerCards(address, participant),
      isMe: address === account?.decodedAddress,
      ...getStatusAndBet(address),
      //     avatar: 'https://avatar.iran.liara.run/public/27',
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

  const winnersHand = [] as Card[];
  const totalPot = bettingBank?.reduce((acc, [, amount]) => acc + Number(amount), 0) || undefined;

  const myWinnerIndex = winners?.findIndex((winner) => winner === account?.decodedAddress);
  const isWinner = myWinnerIndex !== undefined && myWinnerIndex !== -1;
  const myWinnerCashPrize = isWinner && cash_prize ? Number(cash_prize[myWinnerIndex]) : undefined;

  const isMyTurn = turn === account?.decodedAddress && isActiveGame;
  const myCurrentBet = playerSlots?.find(({ isMe }) => isMe)?.bet;

  const [showGameEndModal, setShowGameEndModal] = useState(false);

  useEffect(() => {
    if (!isFinished) return;

    if (!showGameEndModal) {
      setShowGameEndModal(true);
    }

    if (isAdmin && !isRestartGamePending) {
      setTimeout(() => {
        // ! TODO: refetch on error
        void restartGameMessage().then(() => {
          setShowGameEndModal(false);
        });
      }, 3000);
    }
  }, [isFinished, restartGameMessage, isAdmin, showGameEndModal, isRestartGamePending]);

  return (
    <>
      <Header>
        <Button color="contrast" rounded onClick={() => navigate(ROUTES.HOME)}>
          <BackIcon />
        </Button>
      </Header>
      {isMyTurn && <div className={styles.bottomGlow} />}
      <div className={styles.content}>
        <GameBoard totalPot={totalPot} commonCardsFields={commonCardsFields} playerSlots={playerSlots} />
        {isMyTurn && (
          <GameButtons
            currentBet={Number(current_bet || 0)}
            bigBlind={Number(config?.big_blind || 0)}
            myCurrentBet={myCurrentBet || 0}
          />
        )}
        {isMyTurn && <YourTurn />}
        {isWaitingZk && (
          <ZkVerification
            isWaitingShuffleVerification={isWaitingShuffleVerification}
            isWaitingPartialDecryptionsForPlayersCards={isWaitingPartialDecryptionsForPlayersCards}
            isWaitingTableCards={isWaitingTableCards}
          />
        )}
      </div>

      {!isGameStarted && participants && config && (
        <StartGameModal
          isAdmin={isAdmin}
          participants={participants}
          maxPlayers={config.number_of_participants}
          isWaitingStart={isWaitingStart}
        />
      )}

      {showGameEndModal && (
        <GameEndModal
          winner={participants?.find(([address]) => address === account?.decodedAddress)?.[1].name || ''}
          pot={myWinnerCashPrize || 0}
          winnersHand={winnersHand}
          handRank="straight-flush"
          isWinner={isWinner}
        />
      )}
    </>
  );
}
