import { lazy, Suspense } from 'react';

const Identicon = lazy(() => import('@polkadot/react-identicon'));

type AccountIconProps = any;

export function AccountIcon({ children, className, size = 20, theme = 'polkadot', ...rest }: AccountIconProps) {
  return (
    <Suspense fallback={<span style={{ width: size, height: size }} />}>
      <Identicon size={size} theme={theme} className={className} {...rest} />
    </Suspense>
  );
}
