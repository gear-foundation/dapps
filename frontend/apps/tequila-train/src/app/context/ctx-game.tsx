import { createContext, ReactNode, useState } from 'react';
import { DominoTileType, PlayerChoiceType, GameType, IState, PlayersGame } from '../types/game';

const useProgram = () => {
  const [state, setState] = useState<IState>();
  const [game, setGame] = useState<GameType | null>(null);
  const [timer, setTimer] = useState<number>(0);
  const [players, setPlayers] = useState<PlayersGame[]>([]);
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [selectedDomino, setSelectedDomino] = useState<[number, DominoTileType]>();
  const [playerTiles, setPlayerTiles] = useState<DominoTileType[]>();
  const [playerChoice, setPlayerChoice] = useState<PlayerChoiceType>();

  const [previousGame, setPreviousGame] = useState<any>(null);


  return {
    state,
    setState,
    game,
    setGame,
    timer,
    setTimer,
    players,
    setPlayers,
    isAdmin,
    setIsAdmin,
    playerTiles,
    setPlayerTiles,
    selectedDomino,
    setSelectedDomino,
    playerChoice,
    setPlayerChoice,

    previousGame,
    setPreviousGame,
  };
};

export const GameCtx = createContext({} as ReturnType<typeof useProgram>);

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
