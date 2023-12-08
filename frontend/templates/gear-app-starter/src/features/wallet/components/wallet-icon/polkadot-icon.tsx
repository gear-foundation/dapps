import { CSSProperties, useMemo } from 'react';
import { polkadotIcon } from './utils';
import type { Circle } from './types';

function renderCircle({ cx, cy, fill, r }: Circle, key: number) {
  return <circle cx={cx} cy={cy} fill={fill} key={key} r={r} />;
}

export type PolkadotIconProps = {
  address: string;
  className?: string;
  isAlternative?: boolean;
  size?: number;
  style?: CSSProperties;
};

export function PolkadotIcon({ address, className = '', isAlternative = false, size, style = {} }: PolkadotIconProps) {
  const circles = useMemo(() => polkadotIcon(address, { isAlternative }), [address, isAlternative]);

  return (
    <svg className={className} id={address} name={address} style={style} viewBox="0 0 64 64" width={size} height={size}>
      {circles.map(renderCircle)}
    </svg>
  );
}
