import { lazy, Suspense, type ComponentProps } from 'react';

const Identicon = lazy(() => import('@polkadot/react-identicon'));

type IdenticonComponent = (typeof import('@polkadot/react-identicon'))['default'];
type AccountIconProps = ComponentProps<IdenticonComponent>;

export function AccountIcon({ className, size = 20, theme = 'polkadot', ...rest }: AccountIconProps) {
  return (
    <Suspense fallback={<span style={{ width: size, height: size }} />}>
      <Identicon size={size} theme={theme} className={className} {...rest} />
    </Suspense>
  );
}
