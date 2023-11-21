import { Button } from '@/components/ui/button'
import { useAccount } from '@gear-js/react-hooks'
import { useGameMessage, usePending } from '../../hooks'
import { useState } from 'react'

type GameStartButtonProps = BaseComponentProps & {}

export function GameStartButton({ children }: GameStartButtonProps) {
  const { account } = useAccount()
  const message = useGameMessage()
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

  const onGameStart = () => {
    setIsLoading(true)
    setPending(true)
    message(
      {
        StartGame: {
          name:
            account?.meta.name || `Player-${Math.ceil(Math.random() * 1000)}`,
        },
      },
      {
        onError,
        onSuccess,
      }
    )
  }

  return (
    <Button onClick={onGameStart} disabled={pending} isLoading={isLoading}>
      {children}
    </Button>
  )
}
