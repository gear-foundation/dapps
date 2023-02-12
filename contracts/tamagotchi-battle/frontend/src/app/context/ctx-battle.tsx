import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { BattlePlayerType, BattleStateResponse } from 'app/types/battles';
import { HexString } from '@polkadot/util/types';

type Program = {
  battle?: BattleStateResponse;
  setBattle: Dispatch<SetStateAction<BattleStateResponse | undefined>>;
  players: BattlePlayerType[];
  setPlayers: Dispatch<SetStateAction<BattlePlayerType[]>>;
  currentPlayer?: HexString;
  setCurrentPlayer: Dispatch<SetStateAction<HexString | undefined>>;
  roundDamage: number[];
  setRoundDamage: Dispatch<SetStateAction<number[]>>;
};

const useProgram = (): Program => {
  const [players, setPlayers] = useState<BattlePlayerType[]>([]);
  const [battle, setBattle] = useState<BattleStateResponse>();
  const [currentPlayer, setCurrentPlayer] = useState<HexString>();
  const [roundDamage, setRoundDamage] = useState<number[]>([]);

  return {
    battle,
    setBattle,
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
