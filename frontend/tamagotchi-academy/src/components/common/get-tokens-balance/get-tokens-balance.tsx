import { TooltipWrapper, Button, buttonStyles } from '@gear-js/ui'
import { cn } from '@/app/utils'
import { useGetFTBalance } from '@/app/hooks/use-ft-balance'
import { useApp } from '@/app/context'
import { SpriteIcon } from '@/components/ui/sprite-icon'

export const GetTokensBalance = () => {
  const { handler } = useGetFTBalance()
  const { isPending } = useApp()

  return (
    <div>
      <TooltipWrapper text="Get Tokens">
        <Button
          className={cn('group !p-2.5', buttonStyles.light)}
          icon={() => (
            <>
              <SpriteIcon name="test-balance" width={20} height={20} />
              <SpriteIcon
                name="plus"
                width={12}
                height={12}
                className="absolute bottom-2 right-1.5 bg-[#3a3a3a] group-hover:bg-[#6c6c6c] rounded-full transition-colors"
              />
            </>
          )}
          onClick={() => handler()}
          disabled={isPending}
        />
      </TooltipWrapper>
    </div>
  )
}
