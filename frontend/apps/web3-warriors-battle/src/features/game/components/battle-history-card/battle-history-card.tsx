import clsx from 'clsx';
import { Text } from '@/components';
import { AttackIcon, DefenceIcon, DodgeIcon, HealthIcon } from '../../assets/images';
import { HealthIndicator } from '../health-indicator';
import { VariantProps, cva } from 'class-variance-authority';
import { CrossIcon } from '@/assets/images';
import { Move } from '@/app/utils';
import { PlayerState } from '../../types';
import { PlayerStatus } from '../player-status/player-status';
import styles from './battle-history-card.module.scss';

export const variants = cva('', {
  variants: { align: { left: styles.left, right: styles.right } },
  defaultVariants: { align: 'left' },
});

type BattleHistoryCardProps = Omit<PlayerState, 'action'> &
  VariantProps<typeof variants> & {
    onClose?: () => void;
    action: Move | null;
  };

const BattleHistoryCard = ({
  align,
  action,
  attack,
  health,
  defence,
  dodge,
  isDodged,
  receivedDamage,
  name,
  onClose,
}: BattleHistoryCardProps) => {
  const isAlive = health > 0;

  return (
    <div className={variants({ className: styles.wrapper, align })}>
      <div className={styles.header}>
        <Text>
          {action ? (
            <>
              {name} uses <span className={styles[`action-${action}`]}>{action}</span>
            </>
          ) : (
            <span>{name}</span>
          )}
        </Text>

        <PlayerStatus isAlive={isAlive} />
      </div>
      <div className={styles.healthRow}>
        <div className={styles.healthCount}>
          <HealthIcon />
          <Text size="xs" weight="bold">
            {health} {receivedDamage > 0 && <span className={styles.receivedDamage}>(-{receivedDamage})</span>}
          </Text>
        </div>
        <HealthIndicator health={health} prevHealth={health + receivedDamage} size="sm" />

        {Boolean(receivedDamage) && <Text className={styles.recivedText}>received {receivedDamage} damage</Text>}
      </div>
      <div className={styles.stats}>
        <div className={styles.stat}>
          <AttackIcon
            className={clsx(
              styles.icon,
              action === 'Attack' && styles.attack,
              action === 'Ultimate' && styles.ultimate,
            )}
          />
          <Text size="xs" weight="bold">
            {attack} {action === 'Ultimate' && '(x2)'}
          </Text>
        </div>
        <div className={styles.stat}>
          <DefenceIcon className={clsx(styles.icon, action === 'Reflect' && styles.reflect)} />
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
