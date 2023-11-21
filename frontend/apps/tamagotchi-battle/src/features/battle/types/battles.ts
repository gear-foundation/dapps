import { HexString } from '@polkadot/util/types';

export type TamagotchiColor = 'Green' | 'Red' | 'Blue' | 'Purple' | 'Orange' | 'Yellow';
export type PlayerColor = TamagotchiColor;
export type BattleCurrentStateVariants = 'Registration' | 'GameIsOn' | 'WaitNextRound' | 'GameIsOver';
export type BattleRoundMoveVariants = 'Defence' | 'Attack' | 'Skipped';

export type RoundDamageType = [string, string, string, BattleRoundMoveVariants, BattleRoundMoveVariants];

export type BattleStatePair = {
  gameIsOver: boolean;
  moveDeadline: string;
  moves: [];
  ownerIds: HexString[];
  rounds: string;
  tmgIds: HexString[];
  winner: HexString;
};

export type BattleStatePlayer = {
  color: PlayerColor;
  dateOfBirth: string;
  defence: string;
  health: string;
  name: string;
  owner: HexString;
  power: string;
  tmgId: HexString;
  victories: string;
};

export type BattleStateResponse = {
  admins: HexString[];
  completedGames: string;
  currentWinner: HexString;
  pairs: Record<string, BattleStatePair>;
  players: Record<HexString, BattleStatePlayer>;
  playersIds: HexString[];
  currentPlayers: HexString[];
  state: BattleCurrentStateVariants;
};
