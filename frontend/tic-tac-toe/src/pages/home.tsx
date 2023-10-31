import { useAccount } from '@gear-js/react-hooks'
import { useGame } from '@/features/tic-tac-toe/hooks'
import { Game, Welcome } from '@/features/tic-tac-toe'
import { Wallet } from '@/features/wallet'
import { GameStartButton } from '@/features/tic-tac-toe/components/game-start-button'

export default function Home() {
  const { account } = useAccount()
  const { gameState } = useGame()

  return gameState ? (
    <Game game={gameState} />
  ) : (
    <Welcome>
      {!account && <Wallet />}
      {!!account && <GameStartButton>Start the game</GameStartButton>}
    </Welcome>
  )
}
