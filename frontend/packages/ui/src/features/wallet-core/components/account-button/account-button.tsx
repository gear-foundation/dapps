import { ButtonProps as GearButtonProps, buttonStyles } from '@gear-js/ui';
import { Button, ButtonProps as VaraButtonProps } from '@gear-js/vara-ui';
import Identicon from '@polkadot/react-identicon';
import cx from 'clsx';

import styles from './account-button.module.css';

type Props = {
  name: string | undefined;
  address: string;
  block?: boolean;
  onClick: () => void;
};

type VaraProps = Props & {
  color?: VaraButtonProps['color'];
  size?: VaraButtonProps['size'];
};

function VaraAccountButton({ address, name, color = 'dark', size, block, onClick }: VaraProps) {
  return (
    <Button type="button" size={size} color={color} onClick={onClick} block={block}>
      <Identicon value={address} size={16} theme="polkadot" className={styles.icon} />
      <span>{name}</span>
    </Button>
  );
}

type GearProps = Props & {
  color?: GearButtonProps['color'];
  size?: GearButtonProps['size'];
};

function GearAccountButton({ address, name, size = 'medium', color = 'light', block, onClick }: GearProps) {
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
      <Identicon value={address} size={16} theme="polkadot" className={cx(buttonStyles.icon, styles.icon)} />
      <span>{name}</span>
    </button>
  );
}
export { VaraAccountButton, GearAccountButton };
