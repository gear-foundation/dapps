import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, GameCard, Header } from '@/components';
import { Card } from '@/types';

import styles from './combinations.module.scss';

const combinations = [
  {
    name: 'Royal Flush',
    description: 'A, K, Q, J, 10 of the same suit',
    example: ['Ah', 'Kh', 'Qh', 'Jh', 'Th'],
    rank: 1,
  },
  {
    name: 'Straight Flush',
    description: 'Five cards in sequence of the same suit',
    example: ['9h', '8h', '7h', '6h', '5h'],
    rank: 2,
  },
  {
    name: 'Four of a Kind',
    description: 'Four cards of the same rank',
    example: ['Ah', 'As', 'Ad', 'Ac', 'Kh'],
    rank: 3,
  },
  {
    name: 'Full House',
    description: 'Three of a kind plus a pair',
    example: ['Ah', 'As', 'Ad', 'Kh', 'Ks'],
    rank: 4,
  },
  {
    name: 'Flush',
    description: 'Five cards of the same suit',
    example: ['Ah', 'Kh', '7h', '6h', '2h'],
    rank: 5,
  },
  {
    name: 'Straight',
    description: 'Five cards in sequence',
    example: ['Ah', 'Ks', 'Qd', 'Jc', 'Th'],
    rank: 6,
  },
  {
    name: 'Three of a Kind',
    description: 'Three cards of the same rank',
    example: ['Ah', 'As', 'Ad', 'Kh', 'Qs'],
    rank: 7,
  },
  {
    name: 'Two Pair',
    description: 'Two different pairs',
    example: ['Ah', 'As', 'Kh', 'Ks', 'Qd'],
    rank: 8,
  },
  {
    name: 'One Pair',
    description: 'Two cards of the same rank',
    example: ['Ah', 'As', 'Kh', 'Qs', 'Jd'],
    rank: 9,
  },
  {
    name: 'High Card',
    description: 'Highest card when no other hand is made',
    example: ['Ah', 'Ks', 'Qd', 'Jc', '9h'],
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
                    <GameCard key={card} value={card as Card} />
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
