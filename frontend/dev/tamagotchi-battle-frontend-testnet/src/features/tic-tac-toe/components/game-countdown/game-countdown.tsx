import styles from './game-countdown.module.scss'
import Countdown, { CountdownRenderProps } from 'react-countdown'
import { GameMark } from '../game-mark'
import { useGame } from '../../hooks'
import type { IGameInstance } from '../../types'
import { toNumber } from '@/app/utils'

type GameCountdownProps = BaseComponentProps & {
  game: IGameInstance
}

const Clock = ({ minutes, seconds }: CountdownRenderProps) => {
  return (
    <span>
      {`${minutes > 9 ? minutes : '0' + minutes}`}:
      {seconds > 9 ? seconds : '0' + seconds}
    </span>
  )
}

export function GameCountdown({
  game: { playerMark, lastTime },
}: GameCountdownProps) {
  const { setCountdown, countdown } = useGame()

  // useEffect(() => {
  //   console.log(prettyDate(+withoutCommas(lastTime)))
  //   if (lastTime) {
  //     setCountdown((prev) => {
  //       const isNew = prev?.value !== lastTime
  //       return isNew ? { value: lastTime, isActive: isNew } : prev
  //     })
  //   }
  // }, [lastTime])

  return (
    <div className={styles.wrapper}>
      <div>
        <GameMark mark={playerMark} className={styles.mark} />
      </div>
      <div className={styles.text}>Your turn</div>
      {countdown?.isActive && (
        <div className={styles.timer}>
          <Countdown
            date={toNumber(lastTime) + 30000}
            renderer={Clock}
            onComplete={() =>
              setCountdown((prev) => ({
                value: prev ? prev.value : '',
                isActive: false,
              }))
            }
          />
        </div>
      )}
    </div>
  )
}
