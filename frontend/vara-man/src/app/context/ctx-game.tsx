import { createContext, ReactNode, useContext, useState } from 'react'
import { IGame, IGameConfig, IGameInstance, IPlayer, IPlayerInfo } from '@/app/types/game'

const useGameData = () => {
  const [game, setGame] = useState<IGameInstance | null>()
  const [allGames, setAllGames] = useState<IGame[]>()
  const [configState, setConfigState] = useState<IGameConfig | null>()
  const [isAdmin, setIsAdmin] = useState<boolean>(false)
  const [player, setPlayer] = useState<IPlayerInfo>()
  const [allPlayers, setAllPlayers] = useState<IPlayer[]>()

  return {
    game,
    setGame,
    allGames,
    setAllGames,
    configState,
    setConfigState,
    isAdmin,
    setIsAdmin,
    player,
    setPlayer,
    allPlayers,
    setAllPlayers,
  }
}

export const GameCtx = createContext({} as ReturnType<typeof useGameData>)
export const useGame = () => useContext(GameCtx)

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx
  return <Provider value={useGameData()}>{children}</Provider>
}
