import { createContext, useContext, useState } from 'react';
import { BattleStatePlayer, BattleStateResponse, RoundDamageType } from './types/battles';
import { HexString } from '@polkadot/util/types';

const useProgram = () => {
  const [isPending, setIsPending] = useState<boolean>(false);
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [battle, setBattle] = useState<BattleStateResponse>();
  const [rivals, setRivals] = useState<BattleStatePlayer[]>([]);
  const [players, setPlayers] = useState<BattleStatePlayer[]>([]);
  const [currentPairIdx, setCurrentPairIdx] = useState<number>(0);
  const [currentPlayer, setCurrentPlayer] = useState<HexString>();
  const [roundDamage, setRoundDamage] = useState<RoundDamageType>();

  return {
    isPending,
    setIsPending,
    isAdmin,
    setIsAdmin,
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

export const BattleCtx = createContext({} as ReturnType<typeof useProgram>);

export const useBattle = () => useContext(BattleCtx);

export function BattleProvider({ children }: React.PropsWithChildren) {
  const { Provider } = BattleCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
