import clsx from 'clsx';

import { CardBackIcon } from '@/assets/images';
import { Card } from '@/features/zk/api/types';

import styles from './game-card.module.scss';
import { rankIcon, suitLgIcon, suitSmIcon } from './icons';

type Size = 'sm' | 'md' | 'lg';

type Props = {
  className?: string;
  size?: Size;
  isDashed?: boolean;
  isBack?: boolean;
  value: Card | null;
};

const GameCard = ({ className, size = 'md', isDashed = false, isBack = false, value }: Props) => {
  if (isBack) {
    return (
      <div className={clsx(styles.card, styles[size], styles.back, className)}>
        <CardBackIcon />
      </div>
    );
  }

  if (!value) {
    return <div className={clsx(styles.card, styles[size], isDashed && styles.dashed, className)} />;
  }

  console.log('🚀 ~ GameCard ~ value:', value, isDashed);

  const { rank, suit } = value;
  const SuitSmIcon = suitSmIcon[suit];
  const color = suit === 'Clubs' || suit === 'Spades' ? 'black' : 'red';

  const isSm = size === 'sm';
  const isRankWithIcon = rank === 'J' || rank === 'Q' || rank === 'K';
  const SuitLgIcon = isRankWithIcon && !isSm ? rankIcon[color][rank] : suitLgIcon[suit];

  const centerContent = isSm ? rank : <SuitLgIcon />;

  return (
    <div className={clsx(styles.card, styles[size], styles[color], className)}>
      <SuitSmIcon className={styles.suitRightTop} />
      <SuitSmIcon className={styles.suitLeftBottom} />
      <div className={styles.center}>
        <div className={styles.centerInner}>{centerContent}</div>
      </div>
      {isSm ? (
        <SuitLgIcon className={styles.suitSmIcon} />
      ) : (
        <div className={styles.rank}>
          {rank}
          <SuitSmIcon />
        </div>
      )}
    </div>
  );
};

export { GameCard };
