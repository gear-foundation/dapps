import clsx from 'clsx';
import { VariantProps, cva } from 'class-variance-authority';
import { Text } from '@/components';
import { AttackIcon, DefenseIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { ReactComponent as MockAvatarIcon } from './avatar.svg';
import { ReactComponent as VectorIcon } from './vector.svg';
import { HealthIndicator } from '../health-indicator';
import styles from './character-stats.module.scss';

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});

type CharacterStats = {
  icon: React.ReactNode;
  title: string;
  value: number;
};

type CharacterStatsProps = {
  className?: string;
} & VariantProps<typeof variants>;

export const CharacterStats = ({ className, align }: CharacterStatsProps) => {
  // ! TODO: use as props
  const name = 'Player Name 1';
  const currentHealth = 92;
  const attack = 30;
  const deffence = 10;
  const dodge = 30;
  const isActive = false;

  return (
    <div className={variants({ align, className: clsx(styles.container, className) })}>
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
        <MockAvatarIcon />
      </div>
    </div>
  );
};
