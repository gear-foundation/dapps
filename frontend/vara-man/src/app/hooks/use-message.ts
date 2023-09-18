import { useApp } from '@/app/context/ctx-app'
import { useGameMessage } from '@/app/hooks/use-game'
import { IGameLevel } from '@/app/types/game'
import { useNavigate } from 'react-router-dom'
import { useAccount } from '@gear-js/react-hooks'

export function useMessage() {
  const { isPending, setIsPending } = useApp()
  const { account } = useAccount()
  const navigate = useNavigate()
  const handleMessage = useGameMessage()

  const onStart = (level: IGameLevel) => {
    if (account?.decodedAddress) {
      setIsPending(true)

      handleMessage(
        {
          StartGame: {
            level,
            player_address: account.decodedAddress,
          },
        },
        {
          onSuccess: () => {
            setIsPending(false)
            navigate('/game')
          },
          onError: () => setIsPending(false),
        }
      )
    }
  }

  const onClaimReward = (silver_coins: number, gold_coins: number) => {
    setIsPending(true)

    handleMessage(
      {
        ClaimReward: {
          silver_coins,
          gold_coins,
        },
      },
      {
        onSuccess: () => {
          setIsPending(false)
          navigate('/levels')
        },
        onError: () => setIsPending(false),
      }
    )
  }

  return { isPending, onStart, onClaimReward }
}
