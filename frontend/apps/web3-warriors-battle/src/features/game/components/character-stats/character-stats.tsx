import { VariantProps, cva } from 'class-variance-authority';
import clsx from 'clsx';

import { PlayerSettings } from '@/app/utils';
import { Text } from '@/components';

import { AttackIcon, DefenceIcon, DodgeIcon, HealthIcon, SkullBigIcon } from '../../assets/images';
import { Avatar } from '../avatar';
import { CharacterView } from '../character/character';
import { HealthIndicator } from '../health-indicator';

import styles from './character-stats.module.scss';
import VectorIcon from './vector.svg?react';

const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right }, status: { defeated: styles.defeated, alive: null } },
  defaultVariants: { align: 'left', status: 'alive' },
});

type CharacterStatsProps = VariantProps<typeof variants> &
  PlayerSettings & {
    name: string;
    characterView: CharacterView;
    isActive?: boolean;
    className?: string;
  };

export const CharacterStats = ({
  className,
  align,
  name,
  attack,
  health,
  defence,
  dodge,
  characterView,
  isActive = false,
}: CharacterStatsProps) => {
  const status = health === 0 ? 'defeated' : 'alive';

  return (
    <div className={variants({ align, status, className: clsx(styles.container, className) })}>
      <div className={variants({ align, className: clsx(styles.bottom) })}>
        <VectorIcon className={styles.vector} />
        <div className={styles.stats}>
          <div className={styles.stat}>
            <AttackIcon className={styles.attack} />
            <Text size="xs" weight="bold">
              {attack}
            </Text>
          </div>
          <div className={styles.stat}>
            <DefenceIcon className={styles.defence} />
            <Text size="xs" weight="bold">
              {defence}%
            </Text>
          </div>
          <div className={clsx(styles.stat, styles.dodgeStat)}>
            <DodgeIcon className={styles.dodge} />
            <Text size="xs" weight="bold">
              {dodge}% chance
            </Text>
          </div>
        </div>
      </div>
      <div className={variants({ align, className: clsx(styles.top) })}>
        <VectorIcon className={styles.vector} />
        <Text size="lg" weight="bold" className={styles.name}>
          {name}

          {status === 'defeated' && <SkullBigIcon />}
        </Text>
        <div className={styles.health}>
          <div className={styles.healthCount}>
            <HealthIcon />
            <Text size="xs" weight="bold">
              {health}
            </Text>
          </div>
          <HealthIndicator health={health} />
        </div>
      </div>

      <div className={variants({ align, className: clsx(styles.avatar, { [styles.active]: isActive }) })}>
        <Avatar {...characterView} />
      </div>
    </div>
  );
};
