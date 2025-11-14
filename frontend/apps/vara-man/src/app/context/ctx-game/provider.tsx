import { ReactNode, useMemo, useState } from 'react';

import type { IGame, IGameStatus, IPlayer, IPlayerInfo } from '@/app/types/game';

import { GameContext, type GameContextValue } from './context';

type GameProviderProps = {
  children: ReactNode;
};

export function GameProvider({ children }: GameProviderProps) {
  const [game, setGame] = useState<TournamentState | undefined>(undefined);
  const [tournamentGame, setTournamentGame] = useState<TournamentState | undefined>(undefined);
  const [previousGame, setPreviousGame] = useState<TournamentState | null>(null);
  const [allGames, setAllGames] = useState<IGame[] | undefined>(undefined);
  const [configState, setConfigState] = useState<Config | null>(null);
  const [isAdmin, setIsAdmin] = useState(false);
  const [player, setPlayer] = useState<IPlayerInfo | undefined>(undefined);
  const [allPlayers, setAllPlayers] = useState<IPlayer[] | undefined>(undefined);
  const [status, setStatus] = useState<IGameStatus | undefined>(undefined);

  const value = useMemo<GameContextValue>(
    () => ({
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
    }),
    [allGames, configState, game, isAdmin, player, previousGame, status, tournamentGame, allPlayers],
  );

  return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
}
