import clsx from 'clsx';

import styles from './Balance.module.scss';

type Props = BaseComponentProps & {
  SVG: SVGComponent;
  value: string;
  unit?: string;
  isLight?: boolean;
};

function Balance({ SVG, value, unit, className, isLight }: Props) {
  return (
    <span className={clsx(isLight ? styles.wrapperLight : styles.wrapperDark, className)}>
      <SVG />
      <span className={styles.balance}>
        <b className={styles.amount}>{value}</b>
        {unit && <span className={styles.unit}>{unit}</span>}
      </span>
    </span>
  );
}

export { Balance };
