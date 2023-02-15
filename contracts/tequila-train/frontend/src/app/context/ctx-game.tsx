import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { HexString } from '@polkadot/util/types';
import { DominoTileType, GameStateResponse, GameWasmStateResponse, PlayerChoiceType } from '../types/game';

type Program = {
  game?: GameStateResponse;
  setGame: Dispatch<SetStateAction<GameStateResponse | undefined>>;
  gameWasm?: GameWasmStateResponse;
  setGameWasm: Dispatch<SetStateAction<GameWasmStateResponse | undefined>>;
  players: HexString[];
  setPlayers: Dispatch<SetStateAction<HexString[]>>;
  playerTiles?: DominoTileType[];
  setPlayerTiles: Dispatch<SetStateAction<DominoTileType[] | undefined>>;
  selectedDomino?: [number, DominoTileType];
  setSelectedDomino: Dispatch<SetStateAction<[number, DominoTileType] | undefined>>;
  playerChoice?: PlayerChoiceType;
  setPlayerChoice: Dispatch<SetStateAction<PlayerChoiceType | undefined>>;
};

const useProgram = (): Program => {
  const [game, setGame] = useState<GameStateResponse>();
  const [gameWasm, setGameWasm] = useState<GameWasmStateResponse>();
  const [players, setPlayers] = useState<HexString[]>([]);
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
