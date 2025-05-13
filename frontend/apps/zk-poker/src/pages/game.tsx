import { GameBoard } from '@/components';
import { Card, PlayerStatus } from '@/types';

export default function GamePage() {
  const commonCardsFields = new Array(5).fill(null);
  const totalPot = 28.592;
  const playerCards = ['Ah', 'Ks'] as [Card, Card];
  const playerSlots = [
    {
      avatar: 'https://avatar.iran.liara.run/public/27',
      name: 'John Doe 1',
      status: 'waiting' as PlayerStatus,
      chips: 2700,
      cards: null,
      isDiller: true,
    },
    {
      avatar: 'https://avatar.iran.liara.run/public/14',
      name: 'John Doe 2',
      status: 'waiting' as PlayerStatus,
      chips: 10000,
      cards: ['2c', 'Jc'] as [Card, Card],
    },
    {
      avatar: 'https://avatar.iran.liara.run/public/28',
      name: 'John Doe 3',
      status: 'waiting' as PlayerStatus,
      chips: 10000,
      cards: ['3c', 'Jc'] as [Card, Card],
    },
    {
      avatar: 'https://avatar.iran.liara.run/public/29',
      name: 'John Doe 4',
      status: 'waiting' as PlayerStatus,
      chips: 10000,
      cards: ['4c', 'Jc'] as [Card, Card],
    },
    // {
    //   avatar: 'https://avatar.iran.liara.run/public/30',
    //   name: 'John Doe 5',
    //   status: 'waiting' as PlayerStatus,
    //   chips: 10000,
    //   cards: ['5c', 'Jc'] as [Card, Card],
    // },
    // {
    //   avatar: 'https://avatar.iran.liara.run/public/31',
    //   name: 'John Doe 6',
    //   status: 'waiting' as PlayerStatus,
    //   chips: 10000,
    //   cards: ['6c', 'Jc'] as [Card, Card],
    // },
    // {
    //   avatar: 'https://avatar.iran.liara.run/public/32',
    //   name: 'John Doe 7',
    //   status: 'waiting' as PlayerStatus,
    //   chips: 10000,
    //   cards: ['7c', 'Jc'] as [Card, Card],
    // },
    // {
    //   avatar: 'https://avatar.iran.liara.run/public/33',
    //   name: 'John Doe 8',
    //   status: 'waiting' as PlayerStatus,
    //   chips: 10000,
    //   cards: ['8c', 'Jc'] as [Card, Card],
    // },
  ];

  return (
    <GameBoard
      totalPot={totalPot}
      commonCardsFields={commonCardsFields}
      playerSlots={playerSlots}
      playerCards={playerCards}
    />
  );
}
