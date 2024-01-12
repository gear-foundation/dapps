import { Button } from '@gear-js/vara-ui';
import Identicon from '@polkadot/react-identicon';
import cx from 'clsx';

import styles from './account-button.module.css';

type Props = {
  name: string | undefined;
  address: string;
  className?: string;
  onClick: () => void;
};

function AccountButton({ address, name, className, onClick }: Props) {
  return (
    <Button type="button" size="default" color="dark" className={cx(styles.button, className)} onClick={onClick}>
      <span>{name}</span>
      <Identicon value={address} size={16} theme="polkadot" />
    </Button>
  );
}

export { AccountButton };
