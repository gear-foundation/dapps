import { createContext, Dispatch, ReactNode, SetStateAction, useEffect, useState } from 'react';
import { BattlePlayerType, BattleStateResponse } from '../types/battles';
import { HexString } from '@polkadot/util/types';

type Program = {
  battleState?: BattleStateResponse;
  setBattleState: Dispatch<SetStateAction<BattleStateResponse | undefined>>;
  players: BattlePlayerType[];
  setPlayers: Dispatch<SetStateAction<BattlePlayerType[]>>;
  currentPlayer?: HexString;
  setCurrentPlayer: Dispatch<SetStateAction<HexString | undefined>>;
  roundDamage?: number[];
  setRoundDamage: Dispatch<SetStateAction<number[]>>;
};

const useProgram = (): Program => {
  const [players, setPlayers] = useState<BattlePlayerType[]>([]);
  const [battleState, setBattleState] = useState<BattleStateResponse>();
  const [currentPlayer, setCurrentPlayer] = useState<HexString>();
  const [roundDamage, setRoundDamage] = useState<number[]>([]);

  useEffect(() => {
    console.log('round players: ', players);
  }, [players]);
  useEffect(() => {
    console.log({ roundDamage });
  }, [roundDamage]);

  return {
    battleState,
    setBattleState,
    players,
    setPlayers,
    currentPlayer,
    setCurrentPlayer,
    roundDamage,
    setRoundDamage,
  };
};

export const BattleCtx = createContext({} as Program);

export function BattleProvider({ children }: { children: ReactNode }) {
  const { Provider } = BattleCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
