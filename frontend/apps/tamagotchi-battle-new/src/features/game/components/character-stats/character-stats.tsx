import clsx from 'clsx';
import { VariantProps, cva } from 'class-variance-authority';
import { Text } from '@/components';
import { AttackIcon, DefenseIcon, DodgeIcon, HealthIcon, SkullBigIcon } from '../../assets/images';
import { ReactComponent as MockAvatarIcon } from './avatar.svg';
import { ReactComponent as VectorIcon } from './vector.svg';
import { HealthIndicator } from '../health-indicator';
import styles from './character-stats.module.scss';
import { PlayerState } from '../../types';
import { Avatar } from '../avatar';
import { CharacterView } from '../character/character';

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right }, status: { defeated: styles.defeated, alive: null } },
  defaultVariants: { align: 'left', status: 'alive' },
});

type CharacterStatsProps = VariantProps<typeof variants> &
  PlayerState & {
    className?: string;
    characterView: CharacterView;
  };

export const CharacterStats = ({
  className,
  align,
  name,
  attack,
  currentHealth,
  deffence,
  dodge,
  characterView,
}: CharacterStatsProps) => {
  // ! TODO: use as props
  const isActive = false;
  const status = currentHealth === 0 ? 'defeated' : 'alive';

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
            <DefenseIcon className={styles.defense} />
            <Text size="xs" weight="bold">
              {deffence}
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
              {currentHealth}
            </Text>
          </div>
          <HealthIndicator currentHealth={currentHealth} />
        </div>
      </div>

      <div className={variants({ align, className: clsx(styles.avatar, { [styles.active]: isActive }) })}>
        <Avatar {...characterView} />
      </div>
    </div>
  );
};
