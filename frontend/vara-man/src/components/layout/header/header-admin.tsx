import { Icons } from '@/components/ui/icons'
import { useGame } from '@/app/context/ctx-game'
import { useGameMessage } from '@/app/hooks/use-game'
import { useApp } from '@/app/context/ctx-app'

type HeaderAdminProps = BaseComponentProps & {}

export function HeaderAdmin({}: HeaderAdminProps) {
  const { isPending, setIsPending } = useApp()
  const { game } = useGame()
  const handleMessage = useGameMessage()

  const onSuccess = () => setIsPending(false)

  return (
    <>
      {game?.status === 'Paused' && (
        <button
          type="button"
          className="btn btn--primary px-6"
          disabled={isPending}
          onClick={() =>
            handleMessage(
              { ChangeStatus: { Started: null } },
              { onSuccess, onError: onSuccess }
            )
          }
        >
          <Icons.gameJoystick className="w-5 h-5 mr-2" />
          <span>Activate game</span>
        </button>
      )}
      {game?.status === 'Started' && (
        <button
          type="button"
          className="btn btn--theme-red px-6"
          disabled={isPending}
          onClick={() =>
            handleMessage(
              { ChangeStatus: { Paused: null } },
              { onSuccess, onError: onSuccess }
            )
          }
        >
          <Icons.gameJoystick className="w-5 h-5 mr-2" />
          <span>Deactivate game</span>
        </button>
      )}
    </>
  )
}
