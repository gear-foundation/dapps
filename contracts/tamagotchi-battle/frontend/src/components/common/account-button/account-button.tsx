import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { buttonStyles } from '@gear-js/ui';
import { Icon } from '../../ui/icon';
import { decodeAddress } from '@gear-js/api';
import { useAlert } from '@gear-js/react-hooks';

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
  isActive?: boolean;
  simple?: boolean;
};

export const AccountButton = ({ address, name, onClick, isActive, simple }: Props) => {
  const alert = useAlert();
  const onCopy = () => {
    const decodedAddress = decodeAddress(address);

    navigator.clipboard
      .writeText(decodedAddress)
      .then(() => alert.success('Copied'))
      .catch(() => alert.error('Copy error'));
  };

  return (
    <div className="flex items-center gap-4">
      <button
        className={clsx(
          'grow btn gap-2 !justify-start ',
          simple ? 'items-center' : '!grid grid-cols-[20px_1fr_20px]',
          isActive ? buttonStyles.primary : buttonStyles.light,
          buttonStyles.button,
        )}
        onClick={onClick}>
        <Identicon value={address} className={buttonStyles.icon} theme="polkadot" size={20} />
        <span className="truncate w-full">{name}</span>
      </button>
      {!simple && (
        <button
          type="button"
          onClick={onCopy}
          className="shrink-0 grow-0 transition-colors text-white text-opacity-80 hover:text-opacity-100 active:text-opacity-60">
          <Icon name="copy" className="w-5 h-5" />
        </button>
      )}
    </div>
  );
};
