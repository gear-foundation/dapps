import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { DominoTileType, GameWasmStateResponse, IGameState, IPlayer, PlayerChoiceType } from '../types/game';

type Program = {
  game?: IGameState;
  setGame: Dispatch<SetStateAction<IGameState | undefined>>;
  gameWasm?: GameWasmStateResponse;
  setGameWasm: Dispatch<SetStateAction<GameWasmStateResponse | undefined>>;
  players: IPlayer[];
  setPlayers: Dispatch<SetStateAction<IPlayer[]>>;
  playerTiles?: DominoTileType[];
  setPlayerTiles: Dispatch<SetStateAction<DominoTileType[] | undefined>>;
  selectedDomino?: [number, DominoTileType];
  setSelectedDomino: Dispatch<SetStateAction<[number, DominoTileType] | undefined>>;
  playerChoice?: PlayerChoiceType;
  setPlayerChoice: Dispatch<SetStateAction<PlayerChoiceType | undefined>>;
};

const useProgram = (): Program => {
  const [game, setGame] = useState<IGameState>();
  const [gameWasm, setGameWasm] = useState<GameWasmStateResponse>();
  const [players, setPlayers] = useState<IPlayer[]>([]);
  const [selectedDomino, setSelectedDomino] = useState<[number, DominoTileType]>();
  const [playerTiles, setPlayerTiles] = useState<DominoTileType[]>();
  const [playerChoice, setPlayerChoice] = useState<PlayerChoiceType>();

  return {
    game,
    setGame,
    gameWasm,
    setGameWasm,
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

export const GameCtx = createContext({} as Program);

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
