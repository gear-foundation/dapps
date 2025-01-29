import { HexString } from '@polkadot/util/types';
import { HaltReason, Participant as ProgramParticipant, Turn } from '@/app/utils';

type Session = {
  altitude: string;
  weather: string;
  reward: string;
};

type Participant = [HexString, ProgramParticipant];

type Event = {
  participant: HexString;
  name: string | undefined;
  deadRound: boolean;
  firstDeadRound: number;
  fuelLeft: string;
  lastAltitude: string;
  payload: string;
  haltReason: HaltReason | null;
};

type Rank = [HexString, number | string | bigint];

type RankWithName = [`0x${string}`, string, string];

type TurnParticipant = [HexString, Turn];

type Turns = TurnParticipant[][];

type RegistrationStatus =
  | 'registration'
  | 'success'
  | 'error'
  | 'NotEnoughParticipants'
  | 'MaximumPlayersReached'
  | 'PlayerRemoved'
  | 'GameCanceled';

export type { Event, Participant, Turns, Rank, TurnParticipant, Session, RankWithName, RegistrationStatus };
