import { BaseComponentProps } from 'app/types';
import { PolkadotIcon, PolkadotIconProps } from 'features/polkadot-icon';

type Props = BaseComponentProps &
  Omit<PolkadotIconProps, 'address'> & {
    address?: string;
  };

export function AccountIcon({ children, className, size = 20, address, ...rest }: Props) {
  return address ? <PolkadotIcon address={address} size={size} className={className} {...rest} /> : null;
}
