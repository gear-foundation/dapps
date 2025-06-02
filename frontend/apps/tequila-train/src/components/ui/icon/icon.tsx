import { FC } from 'react';

import type { IconProps } from '.';

export const Icon: FC<IconProps> = ({ name, className, section = 'icons', ...props }) => {
  return (
    <svg className={className} {...props}>
      <use href={`/sprites/${section}.svg?sprite#${name}`} />
    </svg>
  );
};
