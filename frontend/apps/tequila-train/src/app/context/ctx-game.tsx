import { createContext, ReactNode, useState } from 'react';
import { DominoTileType, IPlayer, IState, PlayerChoiceType } from '../types/game';

const useProgram = () => {
  const [game, setGame] = useState<IState>();
  const [players, setPlayers] = useState<IPlayer[]>([]);
  const [selectedDomino, setSelectedDomino] = useState<[number, DominoTileType]>();
  const [playerTiles, setPlayerTiles] = useState<DominoTileType[]>();
  const [playerChoice, setPlayerChoice] = useState<PlayerChoiceType>();

  return {
    game,
    setGame,
    players,
    setPlayers,
    playerTiles,
    setPlayerTiles,
    selectedDomino,
    setSelectedDomino,
    playerChoice,
    setPlayerChoice,
  };
};

export const GameCtx = createContext({} as ReturnType<typeof useProgram>);

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
