import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { buttonStyles } from '@gear-js/ui';
import { cn } from '../../../app/utils';

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
      'btn !inline-grid grid-cols-[28px_1fr_28px] !justify-start gap-2.5 w-full px-7 whitespace-nowrap',
      isActive ? 'btn--primary' : buttonStyles.light,
      buttonStyles.button,
    )}
    onClick={onClick}>
    <Identicon
      value={address}
      className={cn(buttonStyles.icon, 'w-7 h-7 -my-2 [&>*]:cursor-pointer')}
      theme="polkadot"
      size={28}
    />
    <span className="block truncate w-full">{name}</span>
  </button>
);
