import clsx from 'clsx'
import styles from './game-field.module.scss'
import { GameCell } from '../game-cell'
import type { IGameInstance } from '../../types'
import { GameMark } from '../game-mark'
import { useGame } from '../../hooks'
import { calculateWinner } from '../../utils'
import { motion } from 'framer-motion'
import { variantsGameMark } from '../../variants'

type GameFieldProps = BaseComponentProps & {
  game: IGameInstance
}

export function GameField({ game }: GameFieldProps) {
  const { countdown } = useGame()
  const board = game.board

  const winnerRow = calculateWinner(board)
  const winnerColor = winnerRow
    ? game.playerMark === board[winnerRow[0][0]]
    : false

  return (
    <div
      className={clsx(
        styles.grid
        // pending && styles.pending
      )}
    >
      {board.map((mark, i) => (
        <GameCell
          key={i}
          disabled={
            Boolean(mark || winnerRow?.length) ||
            !countdown?.isActive ||
            !!game.gameResult
          }
          value={i}
        >
          {mark && (
            <GameMark
              mark={mark}
              className={clsx(
                styles.mark,
                mark === game.playerMark && styles.active
              )}
            />
          )}
        </GameCell>
      ))}
      {winnerRow && (
        <motion.div
          initial="enter"
          animate="center"
          variants={variantsGameMark}
          className={clsx(
            styles.line,
            styles[winnerRow[1]],
            winnerColor && styles['line--primary']
          )}
        />
      )}
    </div>
  )
}
