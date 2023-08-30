import { motion } from 'framer-motion'
import { Button } from '@/components/ui/button'
import styles from './game-cell.module.scss'
import { variantsGameMark } from '../../variants'
import { useGameMessage, usePending } from '../../hooks'
import clsx from 'clsx'
import { useCallback, useState } from 'react'
import { sleep } from '@/app/utils'

type GameFieldProps = BaseComponentProps & {
  disabled?: boolean
  value: number
}

export function GameCell({
  children,
  className,
  disabled,
  value,
}: GameFieldProps) {
  const message = useGameMessage()
  const [loading, setLoading] = useState(false)
  const { setPending } = usePending()

  const onSelectCell = useCallback(async () => {
    if (!loading) {
      setPending(true)
      message(
        { Turn: { step: value } },
        { onError: () => setPending(false), onSuccess: () => setPending(false) }
      )

      setLoading(true)
      await sleep(1)
      setLoading(false)
    }
  }, [loading, message, setPending, value])

  return (
    <Button
      variant="text"
      className={clsx(styles.cell, className)}
      disabled={disabled}
      onClick={onSelectCell}
    >
      <motion.span
        initial="enter"
        animate="center"
        variants={variantsGameMark}
        className={styles.mark}
      >
        {children}
      </motion.span>
    </Button>
  )
}
