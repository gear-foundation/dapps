import { HexString } from '@polkadot/util/types';
import { TamagotchiState } from './lessons';

export type TamagotchiColor = 'Green' | 'Red' | 'Blue' | 'Purple' | 'Orange' | 'Yellow';

export type BattlePlayerResponse = {
  attributes: number[];
  color: TamagotchiColor;
  defence: number;
  health: number;
  power: number;
  owner: HexString;
  tmgId: HexString;
};

export type TamagotchiBattlePlayer = TamagotchiState & {
  attributes: number[];
  energy: number;
  owner: HexString;
  power: number;
  tmgId: HexString;
};

export type BattleStatesList = 'Registration' | 'GameIsOn' | 'WaitNextRound' | 'StartNewRound' | 'GameIsOver';

export type BattleStateResponse = {
  admin: HexString;
  currentWinner: HexString;
  players: Record<HexString, BattlePlayerResponse>;
  playersIds: [];
  round: {
    moves: [];
    players: [];
    tmgIds: [];
  };
  currentTurn: number;
  state: BattleStatesList;
  tmgStoreId: HexString;
};
