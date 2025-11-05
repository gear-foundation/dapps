import type { HexString } from '@polkadot/util/types';
import { useState } from 'react';

import type { BattleStatePlayer, BattleStateResponse, RoundDamageType } from '../types/battles';

export type BattleContextValue = {
  isPending: boolean;
  setIsPending: (value: boolean) => void;
  isAdmin: boolean;
  setIsAdmin: (value: boolean) => void;
  currentPairIdx: number;
  setCurrentPairIdx: (index: number) => void;
  battle: BattleStateResponse | undefined;
  setBattle: (state: BattleStateResponse | undefined) => void;
  players: BattleStatePlayer[];
  setPlayers: (value: BattleStatePlayer[]) => void;
  rivals: BattleStatePlayer[];
  setRivals: (value: BattleStatePlayer[]) => void;
  currentPlayer: HexString | undefined;
  setCurrentPlayer: (value: HexString | undefined) => void;
  roundDamage: RoundDamageType | undefined;
  setRoundDamage: (value: RoundDamageType | undefined) => void;
};

function useBattleState(): BattleContextValue {
  const [isPending, setIsPending] = useState(false);
  const [isAdmin, setIsAdmin] = useState(false);
  const [battle, setBattle] = useState<BattleStateResponse>();
  const [rivals, setRivals] = useState<BattleStatePlayer[]>([]);
  const [players, setPlayers] = useState<BattleStatePlayer[]>([]);
  const [currentPairIdx, setCurrentPairIdx] = useState(0);
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
}

export { useBattleState };
