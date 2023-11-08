import { Button } from '@/components/ui/button'
import { useGame, useGameMessage, usePending } from '../../hooks'
import { useState } from 'react'

type GameStartButtonProps = BaseComponentProps & {
  className?: string
}

export function GameTurnButton({ className }: GameStartButtonProps) {
  const message = useGameMessage()
  const { countdown } = useGame()
  const { pending, setPending } = usePending()
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const onError = () => {
    setPending(false)
    setIsLoading(false)
  }
  const onSuccess = () => {
    setPending(false)
    setIsLoading(false)
  }

  const onNextTurn = () => {
    setIsLoading(true)
    setPending(true)
    message({ Skip: null }, { onError, onSuccess })
  }

  return !countdown?.isActive ? (
    <Button
      onClick={onNextTurn}
      disabled={pending}
      isLoading={isLoading}
      variant="black"
    >
      Skip
    </Button>
  ) : null
}
