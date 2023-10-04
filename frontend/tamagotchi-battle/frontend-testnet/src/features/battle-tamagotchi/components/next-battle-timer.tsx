import { ClockUpdateIcon } from '@/assets/images'
import { getTime, useCountdown } from '@/components/ui/countdown/countdown'
import { PropsWithChildren, useEffect } from 'react'
import { BattleStateResponse } from '@/features/battle-tamagotchi/types/battles'
import { cn } from '@/app/utils'

type NextBattleTimerProps = PropsWithChildren & {
  battle: BattleStateResponse
  className?: string
}

export function NextBattleTimer({
  battle,
  children,
  className,
}: NextBattleTimerProps) {
  const { countdown, trigger } = useCountdown({ time: 0 })

  useEffect(() => {
    const nextGameTime = Math.max(
      ...Object.keys(battle.tournamentsStartTimestamps).map((t) => +t)
    )
    const timeNow = Date.now()
    const nextGameTimeLeft =
      timeNow >= nextGameTime ? 0 : nextGameTime - timeNow
    if (!countdown) trigger(nextGameTimeLeft)
  }, [battle.tournamentsStartTimestamps, countdown, trigger])

  return (
    <div
      className={cn(
        'grid items-center w-fit pl-8 py-6 gap-x-6 border-2 border-primary-600 rounded-xl font-kanit bg-[#2BD07112]',
        className
      )}
    >
      {children}
      <div className="grid grid-cols-[auto_auto] items-center gap-x-4">
        <ClockUpdateIcon />
        <p className="typo-h2 font-normal tracking-[1.6px] min-w-[190px]">
          {getTime(countdown)}
        </p>
      </div>
    </div>
  )
}
