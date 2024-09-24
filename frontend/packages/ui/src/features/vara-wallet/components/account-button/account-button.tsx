import { Button, ButtonProps } from '@gear-js/vara-ui';
import Identicon from '@polkadot/react-identicon';

import styles from './account-button.module.css';

type Props = {
  name: string | undefined;
  address: string;
  color?: ButtonProps['color'];
  size?: ButtonProps['size'];
  block?: ButtonProps['block'];
  onClick: () => void;
};

function AccountButton({ address, name, color = 'dark', size, block, onClick }: Props) {
  return (
    <Button type="button" size={size} color={color} className={styles.button} onClick={onClick} block={block}>
      <Identicon value={address} size={16} theme="polkadot" />
      <span>{name}</span>
    </Button>
  );
}

export { AccountButton };
