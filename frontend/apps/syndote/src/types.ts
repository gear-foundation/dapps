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
  ownerId: HexString;
  name: string;
};

type Players = [HexString, PlayerState][];

type PlayersByStrategyAddress = {
  [key: HexString]: PlayerState;
};

type GameSessionState = {
  GameSession: {
    gameSession: State;
  };
};

type State = {
  adminId: HexString;
  currentTurn: string;
  currentStep: string;
  gameStatus: string & { WaitingForGasForStrategy: HexString };
  numberOfPlayers: string;
  ownership: {};
  players: Players;
  playersQueue: [];
  properties: {};
  propertiesInBank: string;
  round: string;
  winner: HexString;

  entryFee: string | null;
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

type MessagePayload = ({ GameFinished: { winner: HexString } } & { Step: Step }) | string;
type MessageHandlePayload = { Ok: 'GameDeleted' | 'GameWasCancelled' } & { Err: any };
type MessageDetails = {
  to: HexString;
};

type CellValues = {
  heading: string;
  baseRent: string;
  bronze: string;
  silver: string;
  gold: string;
  cell: string;
};

export type {
  PlayerState,
  PlayerType,
  State,
  Step,
  MessagePayload,
  MessageHandlePayload,
  MessageDetails,
  Players,
  Properties,
  CellValues,
  GameSessionState,
  PlayersByStrategyAddress,
};
