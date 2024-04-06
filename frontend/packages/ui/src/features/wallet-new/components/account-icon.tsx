import { lazy, Suspense } from 'react';
import { IdentityProps } from '@polkadot/react-identicon/types';

const Identicon = lazy(() => import('@polkadot/react-identicon'));

type AccountIconProps = {
  className?: string;
  size?: number;
  theme?: string;
} & IdentityProps;

export function AccountIcon({ className, size = 20, theme = 'polkadot', ...rest }: AccountIconProps) {
  return (
    <Suspense fallback={<span style={{ width: size, height: size }} />}>
      <Identicon size={size} theme={theme} className={className} {...rest} />
    </Suspense>
  );
}
