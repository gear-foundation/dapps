import { cn } from '@/app/utils'

type LevelsBackgroundProps = BaseComponentProps & {
  picture: string
}

export function LevelsBackground({
  picture,
  className,
}: LevelsBackgroundProps) {
  return (
    <div
      className={cn(
        'absolute inset-0 -z-1 aspect-[2490/2904] pointer-events-none',
        '-top-20 xxl:-top-15',
        'h-[135%] xxl:h-[116%]',
        'left-[-5%] xl:left-[12.5%] xl2k:left-0',
        className
      )}
    >
      <img
        className="w-full h-full object-contain"
        src={picture}
        alt="Easy"
        width={770}
        height={960}
      />
    </div>
  )
}
