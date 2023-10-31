import { Button } from '@/components/ui/button'
import { useGameMessage, useSubscriptionOnGameMessage } from '../../hooks'
import { useEffect, useState } from 'react'
import { useCheckBalance, useProgramMetadata } from '@/app/hooks'
import metaTxt from '@/features/tic-tac-toe/assets/meta/tic_tac_toe.meta.txt'
import {
  useAccount,
  useAlert,
  useHandleCalculateGas,
} from '@gear-js/react-hooks'
import { ADDRESS } from '../../consts'
import { withoutCommas } from '@/app/utils'

export function GameSkipButton() {
  const meta = useProgramMetadata(metaTxt)
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta)
  const message = useGameMessage()
  const alert = useAlert()
  const { account } = useAccount()
  const { checkBalance } = useCheckBalance()
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage()

  useEffect(() => {
    setIsLoading(isOpened)
  }, [isOpened])

  const onError = () => {
    setIsLoading(false)
    unsubscribe()
  }
  const onSuccess = () => {
    setIsLoading(false)
    console.log('success on skip')
  }

  const onNextTurn = () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return
    }

    const payload = { Skip: null }
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
    <Button
      onClick={() => checkBalance(onNextTurn)}
      isLoading={isLoading}
      variant="black"
    >
      Skip
    </Button>
  )
}
