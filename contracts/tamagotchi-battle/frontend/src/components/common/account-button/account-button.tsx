import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { Button, buttonStyles } from '@gear-js/ui';

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
  isActive?: boolean;
  block?: boolean;
};

export const AccountButton = ({ address, name, onClick, isActive }: Props) => (
  <button
    className={clsx(
      'btn gap-2 w-full !justify-start',
      isActive ? buttonStyles.primary : buttonStyles.light,
      buttonStyles.button,
    )}
    onClick={onClick}>
    <Identicon value={address} className={buttonStyles.icon} theme="polkadot" size={20} />
    <span className="truncate">{name}</span>
  </button>
);
