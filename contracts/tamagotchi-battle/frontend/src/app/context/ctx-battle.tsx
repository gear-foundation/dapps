import { createContext, Dispatch, ReactNode, SetStateAction, useEffect, useState } from 'react';
import { BattlePlayerType, BattleStateResponse } from '../types/battles';

type Program = {
  battleState?: BattleStateResponse;
  setBattleState: Dispatch<SetStateAction<BattleStateResponse | undefined>>;
  players: BattlePlayerType[];
  setPlayers: Dispatch<SetStateAction<BattlePlayerType[]>>;
};

const useProgram = (): Program => {
  const [players, setPlayers] = useState<BattlePlayerType[]>([]);
  const [battleState, setBattleState] = useState<BattleStateResponse>();

  useEffect(() => {
    console.log('round players: ', players);
    console.log('round player 1: ', players[0]);
    console.log('round player 2: ', players[1]);
  }, [players]);

  return {
    battleState,
    setBattleState,
    players,
    setPlayers,
  };
};

export const BattleCtx = createContext({} as Program);

export function BattleProvider({ children }: { children: ReactNode }) {
  const { Provider } = BattleCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
