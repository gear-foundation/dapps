import { HexString } from '@polkadot/util/types';

type Strategy = {
  name: string;
  fuelAmount: string;
  payloadAmount: string;
};

type Session = {
  altitude: string;
  weather: string;
  reward: string;
  sessionId: string;
};

type Participant = [HexString, Strategy];

type Results = {
  turns: Turns;
  rankings: Rank[];
  participants: Participant[];
};

type Event = {
  participant: HexString;
  name: string | undefined;
  deadRound: boolean;
  firstDeadRound: number;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
};

type Rank = [HexString, string];

type RankWithName = [`0x${string}`, string, string];

type State = {
  admin: HexString;
  stage: {
    Registration: Participant[];
    Results: Results;
  };
  master: string;
  altitude: string;
  weather: string;
  reward: string;
  sessionId: string;
  bid: string;
  adminName: string;
};

type LaunchState = {
  Game: State;
};

type TurnParticipant = [
  HexString,
  {
    Alive: {
      fuelLeft: string;
      payloadAmount: string;
    };
  },
];

type Turn = TurnParticipant[];

type Turns = Turn[];

type PlayerStatus = 'Finished' | 'Registered' | null;

type PlayerInfo = {
  PlayerInfo: PlayerStatus;
};

type RegistrationStatus =
  | 'registration'
  | 'success'
  | 'error'
  | 'NotEnoughParticipants'
  | 'MaximumPlayersReached'
  | 'PlayerRemoved'
  | 'GameCanceled';

export type {
  LaunchState,
  State,
  Event,
  Participant,
  Turns,
  Rank,
  TurnParticipant,
  Session,
  PlayerStatus,
  PlayerInfo,
  RankWithName,
  RegistrationStatus,
};
