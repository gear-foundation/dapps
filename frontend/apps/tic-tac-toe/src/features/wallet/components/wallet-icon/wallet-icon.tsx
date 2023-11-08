import { BaseComponentProps } from '@/app/types';
import { PolkadotIcon, PolkadotIconProps } from './polkadot-icon';

type Props = BaseComponentProps &
  Omit<PolkadotIconProps, 'address'> & {
    address?: string;
  };

export function WalletIcon({ children, className, size = 20, address, ...rest }: Props) {
  return <>{address && <PolkadotIcon address={address} size={size} className={className} {...rest} />}</>;
}
