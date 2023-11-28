import styles from './game-reward.module.scss';
import { GameRewardIcon } from '../../assets';
import clsx from 'clsx';
import { PointsBalance } from '@/components/ui/balance';
import { BaseComponentProps } from '@/app/types';

type GameRewardProps = BaseComponentProps & {
  amount: string | null;
};

export function GameReward({ children, amount }: GameRewardProps) {
  return (
    <div className={clsx(styles.wrapper, children)}>
      <div className={styles.text}>
        <GameRewardIcon /> Your rewards:
      </div>
      <PointsBalance value={amount || '0'} className={styles.balance} />
    </div>
  );
}
