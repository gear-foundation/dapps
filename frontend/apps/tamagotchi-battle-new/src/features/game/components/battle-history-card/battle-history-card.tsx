import clsx from 'clsx';
import { Text } from '@/components';
import { AttackIcon, DefenceIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { HealthIndicator } from '../health-indicator';
import { VariantProps, cva } from 'class-variance-authority';
import { CrossIcon } from '@/assets/images';
import { PlayerState } from '../../types';
import { PlayerStatus } from '../player-status/player-status';
import styles from './battle-history-card.module.scss';

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});

type BattleHistoryCardProps = PlayerState &
  VariantProps<typeof variants> & {
    onClose?: () => void;
  };

const BattleHistoryCard = ({
  align,
  action,
  attack,
  health,
  defence,
  dodge,
  isDodged,
  recivedDamage,
  playerId,
  onClose,
}: BattleHistoryCardProps) => {
  const name = 'Player Name 1';

  const isAlive = health > 0;

  return (
    <div className={variants({ className: styles.wrapper, align })}>
      <div className={styles.header}>
        <Text>
          Player {playerId} uses <span className={styles[`action-${action}`]}>{action}</span>
        </Text>

        <PlayerStatus isAlive={isAlive} />
      </div>
      <div className={styles.healthRow}>
        <div className={styles.healthCount}>
          <HealthIcon />
          <Text size="xs" weight="bold">
            {health} {recivedDamage > 0 && <span className={styles.recivedDamage}>(-{recivedDamage})</span>}
          </Text>
        </div>
        <HealthIndicator health={health} prevHealth={health + recivedDamage} size="sm" />

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
          <DefenceIcon className={clsx(styles.icon, action === 'reflect' && styles.reflect)} />
          <Text size="xs" weight="bold">
            {defence}%
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

export { BattleHistoryCard };
