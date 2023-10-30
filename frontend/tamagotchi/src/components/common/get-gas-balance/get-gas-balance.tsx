import { TooltipWrapper, Button, buttonStyles } from '@gear-js/ui'
import { cn } from '@/app/utils'
import { SpriteIcon } from '@/components/ui/sprite-icon'

export const GetGasBalance = () => {
  return (
    <div className="">
      <TooltipWrapper text="Account gas balance">
        <Button
          className={cn('group !p-2.5', buttonStyles.lightGreen)}
          icon={() => (
            <>
              <SpriteIcon name="test-balance" width={20} height={20} />
              {/*<SpriteIcon*/}
              {/*  name="plus"*/}
              {/*  width={12}*/}
              {/*  height={12}*/}
              {/*  className="absolute bottom-2 right-1.5 bg-[#223428] group-hover:bg-[#285b3a] rounded-full transition-colors"*/}
              {/*/>*/}
            </>
          )}
        />
      </TooltipWrapper>
    </div>
  )
}
