import { Button } from '@/components/ui/button'
import { useGameMessage, useSubscriptionOnGameMessage } from '../../hooks'
import { useEffect, useState } from 'react'
import { BaseComponentProps } from '@/app/types'
import { useCheckBalance, useProgramMetadata } from '@/app/hooks'
import {
  useAccount,
  useAlert,
  useHandleCalculateGas,
} from '@gear-js/react-hooks'
import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt'
import { ADDRESS } from '../../consts'
import { withoutCommas } from '@/app/utils'

type GameStartButtonProps = BaseComponentProps & {}

export function GameStartButton({ children }: GameStartButtonProps) {
  const meta = useProgramMetadata(metaTxt)
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta)
  const message = useGameMessage()
  const { account } = useAccount()
  const alert = useAlert()
  const { checkBalance } = useCheckBalance()
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage()

  useEffect(() => {
    console.log({ isOpened })
    setIsLoading(isOpened)
  }, [isOpened])

  const onError = () => {
    setIsLoading(false)
    unsubscribe()
  }
  const onSuccess = () => {
    setIsLoading(false)
    console.log('success on start')
  }

  const onGameStart = () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return
    }
    const payload = { StartGame: null }
    setIsLoading(true)

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const limit = withoutCommas(min_limit as string)

        subscribe()
        message({
          payload,
          gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
          onError,
          onSuccess,
        })
      })
      .catch((error) => {
        console.log(error)
        alert.error('Gas calculation error')
      })
  }

  return (
    <Button onClick={() => checkBalance(onGameStart)} isLoading={isLoading}>
      {children}
    </Button>
  )
}
