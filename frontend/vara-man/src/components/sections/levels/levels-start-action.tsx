import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { cn } from '@/app/utils'
import { Icons } from '@/components/ui/icons'
import { useMessage } from '@/app/hooks/use-level'
import { IGameLevel } from '@/app/types/game'
import { useGame } from '@/app/context/ctx-game'

type LevelsStartActionProps = BaseComponentProps & {
  level: IGameLevel
}

export function LevelsStartAction({
  className,
  level,
}: LevelsStartActionProps) {
  const navigate = useNavigate();
  const { isPending, onStart } = useMessage()
  const { player, game, isAdmin } = useGame()
  const [isDisable, setDisable] = useState(false)

  useEffect(() => {
    if (!player) return

    if (player[1].retries === "3") {
      setDisable(true)
    }

    if (isAdmin) {
      setDisable(false)
    }
  }, [])

  const onClickStart = () => {
    if (game && player) {
      const findGamePlayer = game.games.find((x) => x[0] === player[0])

      if (findGamePlayer) {
        return navigate('/game');
      }

      if (player[1].retries !== "3" || isAdmin) {
        return onStart(level)
      }
    }
  }

  return (
    <div className="pl-36 mt-12">
      <button
        className={cn(
          'btn space-x-2.5',
          className,
          isPending && 'btn--loading'
        )}
        disabled={isDisable || isPending}
        onClick={() => onClickStart()}
      >
        <Icons.gameJoystick className="w-5 h-5" />
        <span>Start game</span>
      </button>
    </div>
  )
}
