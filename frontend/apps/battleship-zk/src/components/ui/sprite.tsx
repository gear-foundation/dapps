import { FC, SVGProps } from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
  name: string;
  section?: string;
};

export const Sprite: FC<IconProps> = ({ name, className, section = 'icons', ...props }) => {
  return (
    <svg className={className} {...props}>
      <use href={`/sprites/${section}.svg?sprite#${name}`} />
    </svg>
  );
};
