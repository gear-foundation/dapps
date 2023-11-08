import { Mark } from '../../types'
import { PlayerIconCircle, PlayerIconCross } from '../../assets'

type PlayerMarkProps = BaseComponentProps & {
  mark: Mark
}

export function GameMark({ mark, className }: PlayerMarkProps) {
  return mark === Mark.X ? (
    <PlayerIconCross className={className} />
  ) : (
    <PlayerIconCircle className={className} />
  )
}
