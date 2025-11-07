import { createContext, useContext } from 'react';
import type { Dispatch, SetStateAction } from 'react';

import type { IGame, IGameStatus, IPlayer, IPlayerInfo } from '@/app/types/game';
import type { Config, TournamentState } from '@/app/utils';

type GameContextValue = {
  game: TournamentState | undefined;
  setGame: Dispatch<SetStateAction<TournamentState | undefined>>;
  tournamentGame: TournamentState | undefined;
  setTournamentGame: Dispatch<SetStateAction<TournamentState | undefined>>;
  previousGame: TournamentState | null;
  setPreviousGame: Dispatch<SetStateAction<TournamentState | null>>;
  allGames: IGame[] | undefined;
  setAllGames: Dispatch<SetStateAction<IGame[] | undefined>>;
  configState: Config | null;
  setConfigState: Dispatch<SetStateAction<Config | null>>;
  isAdmin: boolean;
  setIsAdmin: Dispatch<SetStateAction<boolean>>;
  player: IPlayerInfo | undefined;
  setPlayer: Dispatch<SetStateAction<IPlayerInfo | undefined>>;
  allPlayers: IPlayer[] | undefined;
  setAllPlayers: Dispatch<SetStateAction<IPlayer[] | undefined>>;
  status: IGameStatus | undefined;
  setStatus: Dispatch<SetStateAction<IGameStatus | undefined>>;
};

const GameContext = createContext<GameContextValue | undefined>(undefined);

function useGame() {
  const context = useContext(GameContext);

  if (!context) {
    throw new Error('useGame must be used within a GameProvider');
  }

  return context;
}

export { GameContext, useGame, type GameContextValue };
