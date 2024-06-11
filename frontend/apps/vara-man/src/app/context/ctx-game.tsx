import { createContext, ReactNode, useContext, useState } from 'react';
import { IGame, IGameConfig, IGameStatus, IPlayer, IPlayerInfo, ITournamentGameInstance } from '@/app/types/game';

const useGameData = () => {
  const [game, setGame] = useState<ITournamentGameInstance | undefined>();
  const [tournamentGame, setTournamentGame] = useState<ITournamentGameInstance>();
  const [previousGame, setPreviousGame] = useState<ITournamentGameInstance | null>(null)

  const [allGames, setAllGames] = useState<IGame[]>();
  const [configState, setConfigState] = useState<IGameConfig | null>();
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [player, setPlayer] = useState<IPlayerInfo>();
  const [allPlayers, setAllPlayers] = useState<IPlayer[]>();
  const [status, setStatus] = useState<IGameStatus>();

  return {
    game,
    setGame,
    tournamentGame,
    setTournamentGame,
    previousGame,
    setPreviousGame,
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
    status,
    setStatus,
  };
};

export const GameCtx = createContext({} as ReturnType<typeof useGameData>);
export const useGame = () => useContext(GameCtx);

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx;
  return <Provider value={useGameData()}>{children}</Provider>;
}
