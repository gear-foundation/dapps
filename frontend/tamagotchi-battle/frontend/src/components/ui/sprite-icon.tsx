import { SVGProps } from "react";

export type IconProps = SVGProps<SVGSVGElement> & {
  name: string;
  section?: string;
};

export function SpriteIcon({ name, className, section = "icons", ...props }: IconProps) {
  return (
    <svg className={className} {...props}>
      <use href={`/sprites/${section}.svg?sprite#${name}`} />
    </svg>
  );
};
