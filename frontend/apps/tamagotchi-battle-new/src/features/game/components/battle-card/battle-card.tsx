import clsx from 'clsx';
import { Text } from '@/components';
import { AttackIcon, DefenseIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { HealthIndicator } from '../health-indicator';
import styles from './battle-history-card.module.scss';
import { VariantProps, cva } from 'class-variance-authority';
import { CrossIcon } from '@/assets/images';

export type PlayerState = {
  name: string;
  currentHealth: number;
  attack: number;
  deffence: number;
  dodge: number;
  playerId: number;
  action: 'attack' | 'reflect' | 'ultimate';
  isDodged: boolean;
  recivedDamage: number;
};

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});

type BattleCardProps = PlayerState &
  VariantProps<typeof variants> & {
    onClose?: () => void;
  };

const BattleCard = ({
  align,
  name,
  action,
  attack,
  currentHealth,
  deffence,
  dodge,
  isDodged,
  recivedDamage,
  playerId,
  onClose,
}: BattleCardProps) => {
  return (
    <div className={variants({ className: styles.wrapper, align })}>
      <div className={styles.header}>
        <Text>
          Player {playerId} uses <span className={styles[`action-${action}`]}>{action}</span>
        </Text>
      </div>
      <div className={styles.healthRow}>
        <div className={styles.healthCount}>
          <HealthIcon />
          <Text size="xs" weight="bold">
            {currentHealth} {recivedDamage > 0 && <span className={styles.recivedDamage}>(-{recivedDamage})</span>}
          </Text>
        </div>
        <HealthIndicator currentHealth={currentHealth} prevHealth={currentHealth + recivedDamage} size="sm" />

        <Text className={styles.recivedText}>received 12 damage</Text>
      </div>
      <div className={styles.stats}>
        <div className={styles.stat}>
          <AttackIcon
            className={clsx(
              styles.icon,
              action === 'attack' && styles.attack,
              action === 'ultimate' && styles.ultimate,
            )}
          />
          <Text size="xs" weight="bold">
            {attack} {action === 'ultimate' && '(x2)'}
          </Text>
        </div>
        <div className={styles.stat}>
          <DefenseIcon className={clsx(styles.icon, action === 'reflect' && styles.reflect)} />
          <Text size="xs" weight="bold">
            {deffence}
          </Text>
        </div>
        <div className={clsx(styles.stat, styles.dodgeStat)}>
          <DodgeIcon className={clsx(styles.icon, isDodged && styles.dodge)} />
          <Text size="xs" weight="bold">
            {dodge}% chance
          </Text>
        </div>
        {isDodged && <Text className={styles.dodged}>Dodged</Text>}
      </div>

      {onClose && (
        <button type="button" className={styles.cross} onClick={onClose}>
          <CrossIcon />
        </button>
      )}
    </div>
  );
};

export { BattleCard };
