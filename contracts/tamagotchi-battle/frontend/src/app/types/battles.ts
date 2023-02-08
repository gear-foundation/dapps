import { HexString } from '@polkadot/util/types';
import { TamagotchiState } from './lessons';

export type TamagotchiColor = 'Green' | 'Red' | 'Blue' | 'Purple' | 'Orange' | 'Yellow';

export type BattlePlayerType = {
  attributes: number[];
  color: TamagotchiColor;
  defence: number;
  health: number;
  power: number;
  owner: HexString;
  tmgId: HexString;
  name: string;
  dateOfBirth: number;
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
  players: Record<HexString, BattlePlayerType>;
  playersIds: HexString[];
  round: {
    moves: [];
    players: HexString[];
    tmgIds: HexString[];
  };
  currentTurn: number;
  state: BattleStatesList;
  tmgStoreId: HexString;
};
