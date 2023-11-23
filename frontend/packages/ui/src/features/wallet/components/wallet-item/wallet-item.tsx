import { FunctionComponent, SVGProps } from 'react';
import styles from './wallet-item.module.css';

type Props = {
  icon: FunctionComponent<SVGProps<SVGSVGElement> & { title?: string | undefined }>;
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
