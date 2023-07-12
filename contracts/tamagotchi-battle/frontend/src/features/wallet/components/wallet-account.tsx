import { buttonStyles } from '@gear-js/ui';
import { SpriteIcon } from 'components/ui/sprite-icon';
import { decodeAddress } from '@gear-js/api';
import { copyToClipboard } from '../utils';
import { useAlert } from '@gear-js/react-hooks';
import { lazy } from 'react';
import { cn } from "app/utils";

const Identicon = lazy(() => import('@polkadot/react-identicon'));

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
  isActive?: boolean;
  simple?: boolean;
};

export function WalletAccount({ address, name, onClick, isActive, simple }: Props) {
  const alert = useAlert();
  const onCopy = async () => {
    copyToClipboard(await decodeAddress(address), alert);
  };

  return (
    <div className="flex items-center gap-4">
      <button
        className={cn(
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
          <SpriteIcon name="copy" className="w-5 h-5" />
        </button>
      )}
    </div>
  );
}
