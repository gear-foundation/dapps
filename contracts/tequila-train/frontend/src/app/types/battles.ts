import { HexString } from '@polkadot/util/types';

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

export type BattleCurrentStateVariants = 'Registration' | 'GameIsOn' | 'WaitNextRound' | 'GameIsOver';
export type BattleRoundMoveVariants = 'Defence' | 'Attack';

export type RoundDamageType = [number, number, BattleRoundMoveVariants, BattleRoundMoveVariants];

export type BattleStateResponse = {
  admin: HexString;
  currentWinner: HexString;
  players: Record<HexString, BattlePlayerType>;
  playersIds: HexString[];
  round: {
    moves: BattleRoundMoveVariants[];
    players: HexString[];
    tmgIds: HexString[];
    steps: number;
  };
  currentTurn: number;
  state: BattleCurrentStateVariants;
  tmgStoreId: HexString;
};
