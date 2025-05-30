import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, GameBoard, GameButtons, Header, YourTurn } from '@/components';
import { GameEndModal, StartGameModal } from '@/features/game/components';
import {
  useStatusQuery,
  useParticipantsQuery,
  useEventRegisteredSubscription,
  useConfigQuery,
  useActiveParticipantsQuery,
  useEventPlayerDeletedSubscription,
} from '@/features/game/sails';
import { useZkBackend } from '@/features/zk/hooks';
import { Card, PlayerStatus } from '@/types';

import styles from './game.module.scss';

export default function GamePage() {
  const navigate = useNavigate();
  const { status, refetch: refetchStatus } = useStatusQuery();
  const { account } = useAccount();
  const { participants, refetch: refetchParticipants } = useParticipantsQuery();
  const { activeParticipants: _, refetch: refetchActiveParticipants } = useActiveParticipantsQuery();

  const onPlayersChanged = () => {
    void refetchStatus();
    void refetchParticipants();
    void refetchActiveParticipants();
  };

  useEventRegisteredSubscription({ onData: onPlayersChanged });
  useEventPlayerDeletedSubscription({ onData: onPlayersChanged });

  const { config } = useConfigQuery();

  const isAdmin = account?.decodedAddress === config?.admin_id;
  // console.log('🚀 ~ GamePage ~ activeParticipants:', activeParticipants);

  const getPlayerCards = (_address: string, _participant: Participant) => {
    // if (address === account?.decodedAddress) {
    //   return ['7c', 'Jc'] as [Card, Card];
    // }
    return undefined;
  };

  // console.log('🚀 ~ GamePage ~ participants:', participants);
  const isRegistration = 'registration' in (status || {});
  const isWaitingShuffleVerification = 'waitingShuffleVerification' in (status || {});
  useZkBackend({ isWaitingShuffleVerification });

  const showStartGameModal = isRegistration || isWaitingShuffleVerification;

  // console.log('🚀 ~ GamePage ~ status:', status);
  const playerSlots =
    participants?.map(([address, participant]) => ({
      name: participant.name,
      status: 'waiting' as PlayerStatus,
      chips: Number(participant.balance),
      cards: getPlayerCards(address, participant),
      isMe: address === account?.decodedAddress,
      // cards: [participant.card_1, participant.card_2],
      //     cards: ['7c', 'Jc'] as [Card, Card],
      //     avatar: 'https://avatar.iran.liara.run/public/27',
      //     name: 'John Doe 1',
      //     status: 'bet' as PlayerStatus,
      //     isDiller: true,
      //     bet: 100,
    })) || [];

  // const commonCardsFields = ['Ah', 'Ks', 'Qd', 'Jc', 'Td'] as Card[];
  const commonCardsFields = [null, null, null, null, null] as (Card | null)[];
  const winnersHand = ['Ah', 'Ks', 'Qd', 'Jc', 'Td'] as Card[];
  const totalPot = 28.592;
  const isWinner = true;
  const isMyTurn = false;

  const [showGameEndModal, setShowGameEndModal] = useState(false);

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
        {isMyTurn && <GameButtons />}
        {isMyTurn && <YourTurn />}
      </div>

      {showStartGameModal && participants && config && (
        <StartGameModal isAdmin={isAdmin} participants={participants} maxPlayers={config.number_of_participants} />
      )}

      {showGameEndModal && (
        <GameEndModal
          onClose={() => setShowGameEndModal(false)}
          winner="John Doe"
          pot={totalPot}
          winnersHand={winnersHand}
          handRank="straight-flush"
          isWinner={isWinner}
        />
      )}
    </>
  );
}
