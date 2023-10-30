import clsx from 'clsx'
import styles from './game-field.module.scss'
import { GameCell } from '../game-cell'
import type { IGameInstance } from '../../types'
import { GameMark } from '../game-mark'
import {
  useGame,
  useGameMessage,
  useSubscriptionOnGameMessage,
} from '../../hooks'
import { calculateWinner } from '../../utils'
import { motion } from 'framer-motion'
import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt'
import { variantsGameMark } from '../../variants'
import { BaseComponentProps } from '@/app/types'
import { useEffect } from 'react'
import { useAtom } from 'jotai'
import { stateChangeLoadingAtom } from '../../store'
import {
  useAccount,
  useAlert,
  useHandleCalculateGas,
} from '@gear-js/react-hooks'
import { ADDRESS } from '../../consts'
import { useCheckBalance, useProgramMetadata } from '@/app/hooks'
import { withoutCommas } from '@/app/utils'

type GameFieldProps = BaseComponentProps & {
  game: IGameInstance
}

export function GameField({ game }: GameFieldProps) {
  const { countdown } = useGame()
  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom)
  const board = game.board
  const meta = useProgramMetadata(metaTxt)
  const { account } = useAccount()
  const alert = useAlert()
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta)
  const message = useGameMessage()
  const { checkBalance } = useCheckBalance()
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage()

  const winnerRow = calculateWinner(board)
  const winnerColor = winnerRow
    ? game.playerMark === board[winnerRow[0][0]]
    : false

  const onSelectCell = async (value: number) => {
    if (!meta || !account || !ADDRESS.GAME) {
      return
    }

    const payload = { Turn: { step: value } }

    if (!isLoading) {
      calculateGas(payload)
        .then((res) => res.toHuman())
        .then(({ min_limit }) => {
          const limit = withoutCommas(min_limit as string)

          subscribe()
          message({
            payload,
            gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
            onError: () => {
              unsubscribe()
            },
            onSuccess: () => {
              console.log('success on cell')
            },
          })
        })
        .catch((error) => {
          console.log(error)
          alert.error('Gas calculation error')
        })
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }

  useEffect(() => {
    setIsLoading(isOpened)
  }, [isOpened])

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
          value={i}
          disabled={
            Boolean(mark || winnerRow?.length) ||
            !countdown?.isActive ||
            !!game.gameResult
          }
          isLoading={isLoading}
          onSelectCell={(val) => checkBalance(() => onSelectCell(val))}
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
