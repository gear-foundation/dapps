import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, GameBoard, GameButtons, Header, YourTurn } from '@/components';
import { GameEndModal, StartGameModal } from '@/features/game/components';
import { Card, PlayerStatus } from '@/types';

import styles from './game.module.scss';

const playerSlots = [
  {
    avatar: 'https://avatar.iran.liara.run/public/27',
    name: 'John Doe 1',
    status: 'bet' as PlayerStatus,
    chips: 2700,
    cards: null,
    isDiller: true,
    bet: 100,
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/14',
    name: 'John Doe 2',
    status: 'fold' as PlayerStatus,
    chips: 10000,
    cards: ['2c', 'Jc'] as [Card, Card],
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/28',
    name: 'John Doe 3',
    status: 'winner' as PlayerStatus,
    chips: 10000,
    cards: ['3c', 'Jc'] as [Card, Card],
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/29',
    name: 'John Doe 4',
    status: 'thinking' as PlayerStatus,
    chips: 10000,
    cards: ['4c', 'Jc'] as [Card, Card],
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/30',
    name: 'John Doe 5',
    status: 'waiting' as PlayerStatus,
    chips: 10000,
    cards: ['5c', 'Jc'] as [Card, Card],
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/31',
    name: 'John Doe 6',
    status: 'waiting' as PlayerStatus,
    chips: 10000,
    cards: ['6c', 'Jc'] as [Card, Card],
  },
  {
    avatar: 'https://avatar.iran.liara.run/public/32',
    name: 'John Doe 7',
    status: 'thinking' as PlayerStatus,
    chips: 10000,
    cards: ['7c', 'Jc'] as [Card, Card],
  },
  // {
  //   avatar: 'https://avatar.iran.liara.run/public/33',
  //   name: 'John Doe 8',
  //   status: 'waiting' as PlayerStatus,
  //   chips: 10000,
  //   cards: ['8c', 'Jc'] as [Card, Card],
  // },
];

export default function GamePage() {
  const navigate = useNavigate();
  const commonCardsFields = ['Ah', 'Ks', 'Qd', 'Jc', 'Td'] as Card[];
  const winnersHand = ['Ah', 'Ks', 'Qd', 'Jc', 'Td'] as Card[];
  const totalPot = 28.592;
  const playerCards = ['Ah', 'Ks'] as [Card, Card];
  const isWinner = true;
  const isMyTurn = false;

  const [showStartGameModal, setShowStartGameModal] = useState(false);
  const [showGameEndModal, setShowGameEndModal] = useState(true);

  return (
    <>
      <Header>
        <Button color="contrast" rounded onClick={() => navigate(ROUTES.HOME)}>
          <BackIcon />
        </Button>
      </Header>
      {isMyTurn && <div className={styles.bottomGlow} />}
      <div className={styles.content}>
        <GameBoard
          totalPot={totalPot}
          commonCardsFields={commonCardsFields}
          playerSlots={playerSlots}
          playerCards={playerCards}
        />
        {isMyTurn && <GameButtons />}
        {isMyTurn && <YourTurn />}
      </div>

      {showStartGameModal && (
        <StartGameModal
          totalPlayers={9}
          currentPlayers={5}
          buyIn={10000}
          onStartGame={() => setShowStartGameModal(false)}
          onClose={() => setShowStartGameModal(false)}
        />
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
