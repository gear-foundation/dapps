import clsx from 'clsx';

import type { BaseComponentProps } from '@/app/types';
import { Sprite } from '@/components/ui/sprite';

import styles from './Balance.module.scss';

type Props = BaseComponentProps & {
  icon: string;
  value: string;
  decimal?: string;
  unit?: string;
};

type HOCProps = Omit<Props, 'icon' | 'children'>;

export function Balance({ icon, value, decimal, unit, className }: Props) {
  return (
    <span className={clsx(styles.wrapper, className)}>
      <Sprite name={icon} width={24} height={24} />
      <span className={styles.balance}>
        <b className={styles.amount} dangerouslySetInnerHTML={{ __html: value }} />
        {decimal && <span className={clsx(styles.small, styles.decimal)}>{`.${decimal}`}</span>}
        {unit && <span className={clsx(styles.small, styles.unit)}>{unit}</span>}
      </span>
    </span>
  );
}

export function PointsBalance({ value, unit = 'PPV', className }: HOCProps) {
  return <Balance icon={'points-coin'} value={value} unit={unit} className={className} />;
}
