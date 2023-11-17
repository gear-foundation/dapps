import styles from './Balance.module.scss';
import clsx from 'clsx';
import type { BaseComponentProps } from '@/app/types';
import { Sprite } from '@/components/ui/sprite';

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

export function VaraBalance({ value, unit, className }: HOCProps) {
  const v = value.split('.');
  return (
    <Balance
      icon={unit?.toLowerCase() === 'vara' ? 'vara-coin' : 'tvara-coin'}
      value={v[0].replaceAll(/,|\s/g, '&thinsp;')}
      decimal={v[1]}
      unit={unit}
      className={className}
    />
  );
}

export function PointsBalance({ value, unit = 'PPV', className }: HOCProps) {
  return <Balance icon={'points-coin'} value={value} unit={unit} className={className} />;
}
