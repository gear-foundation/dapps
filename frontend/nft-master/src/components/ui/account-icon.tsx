import { lazy, PropsWithChildren, Suspense } from 'react';
import { IdentityProps } from '@polkadot/react-identicon/types';

const Identicon = lazy(() => import('@polkadot/react-identicon'));

type AccountIconProps = IdentityProps &
  PropsWithChildren & {
    className?: string;
  };

export function AccountIcon({ children, className, size = 20, theme = 'polkadot', ...rest }: AccountIconProps) {
  return (
    <Suspense fallback={<span style={{ width: size, height: size }} />}>
      <Identicon size={size} theme={theme} className={className} {...rest} />
    </Suspense>
  );
}
