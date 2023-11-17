import { cx } from '@/utils';
import styles from './NftSpec.module.scss';
import { NftSpecProps } from './NftSpec.interfaces';

function NftSpec({ title, value, icon }: NftSpecProps) {
  return (
    <div className={cx(styles.container)}>
      <div className={cx(styles['icon-wrapper'])}>
        <img src={icon} alt="icon" />
      </div>
      <div className={cx(styles.content)}>
        <div className={cx(styles.title)}>{title}</div>
        <div className={cx(styles.value)}>{value}</div>
      </div>
    </div>
  );
}

export { NftSpec };
