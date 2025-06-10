import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, GameCard, Header } from '@/components';
import { Card } from '@/features/zk/api/types';

import styles from './combinations.module.scss';

type Combination = {
  name: string;
  description: string;
  example: Card[];
  rank: number;
};

const combinations: Combination[] = [
  {
    name: 'Royal Flush',
    description: 'A, K, Q, J, 10 of the same suit',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Hearts', rank: 'Q' },
      { suit: 'Hearts', rank: 'J' },
      { suit: 'Hearts', rank: '10' },
    ],
    rank: 1,
  },
  {
    name: 'Straight Flush',
    description: 'Five cards in sequence of the same suit',
    example: [
      { suit: 'Hearts', rank: '9' },
      { suit: 'Hearts', rank: '8' },
      { suit: 'Hearts', rank: '7' },
      { suit: 'Hearts', rank: '6' },
      { suit: 'Hearts', rank: '5' },
    ],
    rank: 2,
  },
  {
    name: 'Four of a Kind',
    description: 'Four cards of the same rank',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'A' },
      { suit: 'Diamonds', rank: 'A' },
      { suit: 'Clubs', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
    ],
    rank: 3,
  },
  {
    name: 'Full House',
    description: 'Three of a kind plus a pair',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'A' },
      { suit: 'Diamonds', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Spades', rank: 'K' },
    ],
    rank: 4,
  },
  {
    name: 'Flush',
    description: 'Five cards of the same suit',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Hearts', rank: '7' },
      { suit: 'Hearts', rank: '6' },
      { suit: 'Hearts', rank: '2' },
    ],
    rank: 5,
  },
  {
    name: 'Straight',
    description: 'Five cards in sequence',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'K' },
      { suit: 'Diamonds', rank: 'Q' },
      { suit: 'Hearts', rank: 'J' },
      { suit: 'Hearts', rank: '10' },
    ],
    rank: 6,
  },
  {
    name: 'Three of a Kind',
    description: 'Three cards of the same rank',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'A' },
      { suit: 'Diamonds', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Spades', rank: 'Q' },
    ],
    rank: 7,
  },
  {
    name: 'Two Pair',
    description: 'Two different pairs',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Spades', rank: 'K' },
      { suit: 'Diamonds', rank: 'Q' },
    ],
    rank: 8,
  },
  {
    name: 'One Pair',
    description: 'Two cards of the same rank',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'A' },
      { suit: 'Hearts', rank: 'K' },
      { suit: 'Spades', rank: 'Q' },
      { suit: 'Diamonds', rank: 'J' },
    ],
    rank: 9,
  },
  {
    name: 'High Card',
    description: 'Highest card when no other hand is made',
    example: [
      { suit: 'Hearts', rank: 'A' },
      { suit: 'Spades', rank: 'K' },
      { suit: 'Diamonds', rank: 'Q' },
      { suit: 'Clubs', rank: 'J' },
      { suit: 'Hearts', rank: '9' },
    ],
    rank: 10,
  },
];

export default function CombinationsPage() {
  const navigate = useNavigate();

  return (
    <>
      <Header>
        <Button color="contrast" rounded onClick={() => navigate(ROUTES.HOME)}>
          <BackIcon />
        </Button>
      </Header>

      <div className={styles.container}>
        <h1 className={styles.title}>Poker Hand Rankings</h1>
        <p className={styles.description}>From highest to lowest, here are the winning combinations in poker:</p>

        <div className={styles.combinations}>
          {combinations.map((combination) => (
            <div key={combination.name} className={styles.combination}>
              <div className={styles.combinationHeader}>
                <span className={styles.rank}>#{combination.rank}</span>
                <h2 className={styles.name}>{combination.name}</h2>
              </div>
              <p className={styles.description}>{combination.description}</p>
              <div className={styles.example}>
                <span className={styles.exampleLabel}>Example:</span>
                <div className={styles.cards}>
                  {combination.example.map((card) => (
                    <GameCard key={`${card.suit}-${card.rank}`} value={card} />
                  ))}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
