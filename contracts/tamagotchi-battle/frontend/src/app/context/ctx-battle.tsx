import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { BattleStatePlayer, BattleStateResponse, RoundDamageType } from 'app/types/battles';
import { HexString } from '@polkadot/util/types';

type Program = {
  battle?: BattleStateResponse;
  setBattle: Dispatch<SetStateAction<BattleStateResponse | undefined>>;
  currentPairIdx: number;
  setCurrentPairIdx: Dispatch<SetStateAction<number>>;
  players: BattleStatePlayer[];
  setPlayers: Dispatch<SetStateAction<BattleStatePlayer[]>>;
  rivals: BattleStatePlayer[];
  setRivals: Dispatch<SetStateAction<BattleStatePlayer[]>>;
  currentPlayer?: HexString;
  setCurrentPlayer: Dispatch<SetStateAction<HexString | undefined>>;
  roundDamage?: RoundDamageType;
  setRoundDamage: Dispatch<SetStateAction<RoundDamageType | undefined>>;
};

const useProgram = (): Program => {
  const [battle, setBattle] = useState<BattleStateResponse>();
  const [rivals, setRivals] = useState<BattleStatePlayer[]>([]);
  const [players, setPlayers] = useState<BattleStatePlayer[]>([]);
  const [currentPairIdx, setCurrentPairIdx] = useState<number>(0);
  const [currentPlayer, setCurrentPlayer] = useState<HexString>();
  const [roundDamage, setRoundDamage] = useState<RoundDamageType>();

  return {
    currentPairIdx,
    setCurrentPairIdx,
    battle,
    setBattle,
    players,
    setPlayers,
    rivals,
    setRivals,
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
