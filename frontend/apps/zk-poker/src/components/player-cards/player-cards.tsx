import clsx from 'clsx';

import { CardBackDillerIcon } from '@/assets/images';
import { Card } from '@/types';

import { GameCard } from '../game-card';

import styles from './player-cards.module.scss';

type Props = {
  cards: [Card, Card] | null;
  isDiller?: boolean;
  position: {
    top?: number;
    left?: number;
    right?: number;
  };
};

const PlayerCards = ({ isDiller, cards, position }: Props) => {
  return (
    <div className={clsx(styles.playerCards, !cards && styles.back)} style={{ ...position }}>
      <GameCard value={cards?.[0] ?? null} size="sm" isBack={!cards?.[0]} />
      <GameCard value={cards?.[1] ?? null} size="sm" isBack={!cards?.[1]} />
      {isDiller && (
        <div className={styles.diller}>
          <CardBackDillerIcon />
        </div>
      )}
    </div>
  );
};

export { PlayerCards };
