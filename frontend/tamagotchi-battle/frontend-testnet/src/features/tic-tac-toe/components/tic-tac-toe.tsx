import { useGame } from '../hooks'
import { Game } from './game'
import { Welcome } from './welcome'
import { useAccount } from '@gear-js/react-hooks'
import { GameStartButton } from '@/features/tic-tac-toe/components/game-start-button'
import { Wallet } from '@/features/wallet'

export function TicTacToe() {
  const { account } = useAccount()
  const { gameState, gameConfig } = useGame()

  return gameState && gameConfig ? (
    <Game game={gameState} config={gameConfig} />
  ) : (
    <Welcome>
      {!!account ? (
        <GameStartButton>Start the game</GameStartButton>
      ) : (
        <Wallet account={account} isReady />
      )}
    </Welcome>
  )
}
