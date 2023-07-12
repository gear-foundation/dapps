import { HexString } from '@polkadot/util/types';

type PlayerState = {
  position: string;
  balance: string;
  debt: string;
  inJail: boolean;
  round: string;
  cells: [];
  penalty: string;
  lost: boolean;
};

type Players = [HexString, PlayerState][];

type State = {
  admin: HexString;
  currentTurn: string;
  currentStep: string;
  gameStatus: string;
  numberOfPlayers: string;
  ownership: {};
  players: Players;
  playersQueue: [];
  properties: {};
  propertiesInBank: string;
  round: string;
  winner: HexString;
};

type PlayerType = {
  color: 'pink' | 'purple' | 'green' | 'yellow';
  address: string;
  balance: string;
};

type Properties = [HexString, ['Bronze' | 'Silver' | 'Gold'], string, string][];

type Step = {
  currentStep: string;
  currentPlayer: HexString;
  players: Players;
  properties: Properties;
  ownership: HexString[];
};

type MessagePayload = { GameFinished: { winner: HexString } } | { Step: Step } | string;

type CellValues = {
  heading: string;
  baseRent: string;
  bronze: string;
  silver: string;
  gold: string;
  cell: string;
};

export type { PlayerState, PlayerType, State, Step, MessagePayload, Players, Properties, CellValues };
