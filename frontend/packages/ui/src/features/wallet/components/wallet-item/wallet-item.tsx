import { FunctionComponent, SVGProps } from 'react';
import styles from './wallet-item.module.css';

type Props = {
  SVG: FunctionComponent<SVGProps<SVGSVGElement> & { title?: string | undefined }>;
  name: string;
};

function WalletItem({ SVG, name }: Props) {
  return (
    <span className={styles.wallet}>
      <SVG className={styles.icon} />
      {name}
    </span>
  );
}

export { WalletItem };
