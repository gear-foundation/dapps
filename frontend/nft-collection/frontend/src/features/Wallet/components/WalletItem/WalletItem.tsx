import styles from './WalletItem.module.scss';
import { WalletItemProps } from './WalletItem.interfaces';
import { cx } from '@/utils';

function WalletItem({ icon, name }: WalletItemProps) {
  return (
    <span className={cx(styles.wallet)}>
      <img src={icon} alt={name} className={styles.icon} />
      {name}
    </span>
  );
}

export { WalletItem };
