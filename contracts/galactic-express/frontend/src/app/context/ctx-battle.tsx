import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { BattlePlayerType, BattleStateResponse, RoundDamageType } from 'app/types/battles';
import { HexString } from '@polkadot/util/types';

type Program = {
  battle?: BattleStateResponse;
  setBattle: Dispatch<SetStateAction<BattleStateResponse | undefined>>;
  players: BattlePlayerType[];
  setPlayers: Dispatch<SetStateAction<BattlePlayerType[]>>;
  rivals: BattlePlayerType[];
  setRivals: Dispatch<SetStateAction<BattlePlayerType[]>>;
  currentPlayer?: HexString;
  setCurrentPlayer: Dispatch<SetStateAction<HexString | undefined>>;
  roundDamage?: RoundDamageType;
  setRoundDamage: Dispatch<SetStateAction<RoundDamageType | undefined>>;
};

const useProgram = (): Program => {
  const [battle, setBattle] = useState<BattleStateResponse>();
  const [players, setPlayers] = useState<BattlePlayerType[]>([]);
  const [rivals, setRivals] = useState<BattlePlayerType[]>([]);
  const [currentPlayer, setCurrentPlayer] = useState<HexString>();
  const [roundDamage, setRoundDamage] = useState<RoundDamageType>();

  return {
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
