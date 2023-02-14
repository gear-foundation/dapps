import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { HexString } from '@polkadot/util/types';
import { GameStateResponse, GameWasmStateResponse } from '../types/game';

type Program = {
  game?: GameStateResponse;
  setGame: Dispatch<SetStateAction<GameStateResponse | undefined>>;
  gameWasm?: GameWasmStateResponse;
  setGameWasm: Dispatch<SetStateAction<GameWasmStateResponse | undefined>>;
  players: HexString[];
  setPlayers: Dispatch<SetStateAction<HexString[]>>;
  currentPlayer?: number;
  setCurrentPlayer: Dispatch<SetStateAction<number | undefined>>;
};

const useProgram = (): Program => {
  const [game, setGame] = useState<GameStateResponse>();
  const [gameWasm, setGameWasm] = useState<GameWasmStateResponse>();
  const [players, setPlayers] = useState<HexString[]>([]);
  const [currentPlayer, setCurrentPlayer] = useState<number>();

  return {
    game,
    setGame,
    gameWasm,
    setGameWasm,
    players,
    setPlayers,
    currentPlayer,
    setCurrentPlayer,
  };
};

export const GameCtx = createContext({} as Program);

export function GameProvider({ children }: { children: ReactNode }) {
  const { Provider } = GameCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
