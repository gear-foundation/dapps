import clsx from 'clsx';

import { CardBackDillerIcon } from '@/assets/images';
import { Card } from '@/features/zk/api/types';

import { FlipCard } from '../flip-card';

import styles from './player-cards.module.scss';

type Props = {
  cards?: [Card, Card] | null;
  isDiller?: boolean;
  top: number;
  side: 'left' | 'right' | 'top';
};

const PlayerCards = ({ isDiller, cards, top, side }: Props) => {
  return (
    <div className={clsx(styles.playerCards, styles[side], !cards && styles.back)} style={{ top }}>
      <FlipCard value={cards?.[0] ?? null} size="sm" />
      <FlipCard value={cards?.[1] ?? null} size="sm" delay={100} />

      {isDiller && (
        <div className={styles.diller}>
          <CardBackDillerIcon />
        </div>
      )}
    </div>
  );
};

export { PlayerCards };
