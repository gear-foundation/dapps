import { cn } from '@/app/utils'
import { BattleStatesList } from '@/app/types/battles'
import { SpriteIcon } from '@/components/ui/sprite-icon'

type Props = {
  health?: number
  isWinner?: boolean
  state: BattleStatesList
  isReverse?: boolean
  children?: JSX.Element
}
export const TamagotchiBattleTopStats = ({
  isWinner,
  state,
  health,
  isReverse,
  children,
}: Props) => {
  return (
    <div
      className={cn(
        'basis-[445px] flex gap-6 items-center',
        isReverse && 'flex-row-reverse'
      )}
    >
      <div className="relative flex flex-col items-center w-fit">
        <div
          className={cn(
            'relative w-15 xl:w-25 aspect-square rounded-full overflow-hidden ring-2 ring-opacity-50',
            state === 'GameIsOver'
              ? isWinner
                ? 'bg-primary ring-primary'
                : 'bg-error ring-error'
              : 'bg-white ring-white'
          )}
        >
          {children}
        </div>
        {/*<div className="absolute top-[calc(100%-8px)] inline-flex gap-2 items-center py-1 px-4 tracking-widest font-kanit font-semibold text-base leading-5 bg-[#404040] rounded-lg">*/}
        {/*  <SpriteIcon name="wins" className="w-5 h-5" /> 10*/}
        {/*</div>*/}
      </div>
      <div className="w-full max-w-[300px] space-y-3">
        {/*<div className="flex gap-2 items-center justify-center py-0.5 px-4 bg-[#1852FF] rounded-xl">*/}
        {/*  <SpriteIcon name="armor" className="w-3.5 h-3.5" />*/}
        {/*  <span className="font-kanit font-medium leading-5">{Math.round(warriors.p1.power / 1000)} / 10</span>*/}
        {/*</div>*/}
        <div className="relative py-0.5 px-4 rounded-xl overflow-hidden bg-white/10">
          <div
            className={cn(
              'absolute inset-0 -z-1 rounded-xl',
              state === 'GameIsOver'
                ? isWinner
                  ? 'bg-primary'
                  : 'bg-error'
                : 'bg-primary'
            )}
            style={{ width: `${health}%` }}
          />
          <div className="flex gap-2 items-center justify-center">
            <SpriteIcon name="health" className="w-3.5 h-3.5" />
            <span className="font-kanit font-medium leading-5">
              {health} / 100
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}
