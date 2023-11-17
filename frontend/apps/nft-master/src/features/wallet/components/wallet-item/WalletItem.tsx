import { SVGComponent } from 'types';
import styles from './WalletItem.module.scss';

type Props = {
  icon: SVGComponent;
  name: string;
};

function WalletItem({ icon: Icon, name }: Props) {
  return (
    <span className={styles.wallet}>
      <Icon className={styles.icon} />
      {name}
    </span>
  );
}

export { WalletItem };
