import { createContext, ReactNode, useContext, useState } from 'react'
import { IGameState, IPlayer } from '@/app/types/game'

const useGameData = () => {
  const [game, setGame] = useState<IGameState>()
  const [isAdmin, setIsAdmin] = useState<boolean>(false)
  const [player, setPlayer] = useState<IPlayer>()

  return {
    game,
    setGame,
    isAdmin,
    setIsAdmin,
    player,
    setPlayer,
  }
}

export const GameCtx = createContext({} as ReturnType<typeof useGameData>)
export const useGame = () => useContext(GameCtx)

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx
  return <Provider value={useGameData()}>{children}</Provider>
}
