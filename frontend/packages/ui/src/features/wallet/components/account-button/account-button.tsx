import { ButtonProps, buttonStyles } from '@gear-js/ui';
import Identicon from '@polkadot/react-identicon';
import cx from 'clsx';

import styles from './account-button.module.css';

type Props = {
  name: string | undefined;
  address: string;
  color?: ButtonProps['color'];
  size?: ButtonProps['size'];
  block?: ButtonProps['block'];
  onClick: () => void;
};

function AccountButton({ address, name, onClick, size = 'medium', color = 'light', block }: Props) {
  return (
    <button
      type="button"
      className={cx(
        buttonStyles.button,
        buttonStyles.noWrap,
        buttonStyles[size],
        buttonStyles[color],
        block && buttonStyles.block,
        styles.button,
      )}
      onClick={onClick}>
      <Identicon value={address} size={16} theme="polkadot" className={buttonStyles.icon} /> {name}
    </button>
  );
}

export { AccountButton };
