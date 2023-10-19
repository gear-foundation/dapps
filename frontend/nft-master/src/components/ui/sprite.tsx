import { SVGProps } from 'react'

type IconProps = SVGProps<SVGSVGElement> & {
  name: string
  section?: string
  size?: number | string
}

export function Sprite({ name, className, section = 'icons', size, ...props }: IconProps) {
  return (
    <svg className={className} width={size || props.width} height={size || props.height} {...props}>
      <use href={`/sprites/${section}.svg?sprite#${name}`} />
    </svg>
  )
}
