import clsx from 'clsx';
import { useEffect, useState } from 'react';

import { Card } from '@/features/zk/api/types';

import { GameCard } from '../game-card';

import styles from './flip-card.module.scss';

type Size = 'sm' | 'md' | 'lg';

type Props = {
  className?: string;
  size?: Size;
  value: Card | null;
  duration?: number;
  delay?: number;
  isDashed?: boolean;
};

const FlipCard = ({ className, size = 'md', value, duration = 600, delay = 0, isDashed = false }: Props) => {
  const [isFlipped, setIsFlipped] = useState(true);

  useEffect(() => {
    setTimeout(() => {
      setIsFlipped(!value);
    }, delay);
  }, [value, delay]);

  return (
    <div
      className={clsx(styles.flipCard, styles[size], isFlipped && styles.flipped, className)}
      style={{ '--flip-duration': `${duration}ms` } as React.CSSProperties}>
      <div className={styles.flipCardInner}>
        <div className={styles.flipCardFront}>
          <GameCard value={value} size={size} isDashed={isDashed} />
        </div>
        <div className={styles.flipCardBack}>
          <GameCard value={null} size={size} isBack={!isDashed || !!value} isDashed={isDashed} />
        </div>
      </div>
    </div>
  );
};

export { FlipCard };
