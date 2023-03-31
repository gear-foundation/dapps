import { HexString } from '@polkadot/util/types';

type PlayerState = {
  position: number;
  balance: number;
  debt: number;
  inJail: boolean;
  round: number;
  cells: [];
  penalty: number;
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
  round: number;
  winner: HexString;
};

type PlayerType = {
  color: 'pink' | 'purple' | 'green' | 'yellow';
  address: string;
  balance: number;
};

type Properties = [HexString, ['Bronze' | 'Silver' | 'Gold'], number, number][];

type Step = {
  currentStep: number;
  currentPlayer: HexString;
  players: Players;
  properties: Properties;
  ownership: HexString[];
};

type MessagePayload = { GameFinished: { winner: HexString } } | { Step: Step } | string;

type CellValues = {
  heading: string;
  baseRent: number;
  bronze: number;
  silver: number;
  gold: number;
  cell: number;
};

export type { PlayerState, PlayerType, State, Step, MessagePayload, Players, Properties, CellValues };
