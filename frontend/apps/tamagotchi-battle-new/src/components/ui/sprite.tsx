import { FC, SVGProps } from 'react';

type IconProps = SVGProps<SVGSVGElement> & {
  name: string;
  section?: string;
  size?: number;
};

export const Sprite: FC<IconProps> = ({ name, className, section = 'icons', size, ...props }) => {
  return (
    <svg className={className} width={size || props.width} height={size || props.height} {...props}>
      <use href={`/sprites/${section}.svg?sprite#${name}`} />
    </svg>
  );
};
