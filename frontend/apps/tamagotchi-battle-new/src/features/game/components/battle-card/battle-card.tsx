import { VariantProps, cva } from 'class-variance-authority';
import clsx from 'clsx';
import { Text } from '@/components';
import { AttackIcon, CupStarIcon, DefenseIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { PlayerState } from '../../types';
import { Avatar } from '../avatar';
import { CharacterView } from '../character/character';
import styles from './battle-card.module.scss';

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});

type BattleCardProps = PlayerState &
  VariantProps<typeof variants> & {
    winsCount?: number;
    characterView: CharacterView;
  };

const BattleCard = ({
  align,
  name,
  attack,
  currentHealth,
  deffence,
  dodge,
  winsCount = 0,
  characterView,
}: BattleCardProps) => {
  return (
    <div className={variants({ className: styles.wrapper, align })}>
      <Avatar size="sm" {...characterView} />
      <div className={styles.info}>
        <div className={styles.header}>
          <Text>{name}</Text>
          <div className={styles.winsCount}>
            <CupStarIcon />
            <Text weight="bold" size="xs">
              {winsCount}
            </Text>
          </div>
        </div>
        <div className={styles.stats}>
          <div className={styles.stat}>
            <HealthIcon />
            <Text size="xs" weight="bold">
              {currentHealth}
            </Text>
          </div>
          <div className={clsx(styles.stat, styles.attackStat)}>
            <AttackIcon className={clsx(styles.icon, styles.attack)} />
            <Text size="xs" weight="bold">
              {attack}
            </Text>
          </div>
          <div className={styles.stat}>
            <DefenseIcon className={clsx(styles.icon, styles.reflect)} />
            <Text size="xs" weight="bold">
              {deffence}%
            </Text>
          </div>
          <div className={clsx(styles.stat, styles.dodgeStat)}>
            <DodgeIcon className={clsx(styles.icon, styles.dodge)} />
            <Text size="xs" weight="bold">
              {dodge}%
            </Text>
          </div>
        </div>
      </div>
    </div>
  );
};

export { BattleCard };
