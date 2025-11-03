import { HexString } from '@polkadot/util/types';

import { Gear, PlayerInfoState } from '@/app/utils';

type Players = Array<[HexString, PlayerInfoState]>;

type PlayersByStrategyAddress = {
  [key: HexString]: PlayerInfoState;
};

type PlayerType = {
  color: 'pink' | 'purple' | 'green' | 'yellow';
  address: string;
  balance: number;
};

type Properties = Array<[HexString, Array<Gear>, number, number] | null>;

type Step = {
  properties: Properties;
  current_player: HexString;
  players: Array<[HexString, PlayerInfoState]>;
  ownership: Array<HexString>;
  current_step: number | string | bigint;
};

type MessagePayload = ({ GameFinished: { winner: HexString } } & { Step: Step }) | string;
type MessageHandlePayload = { Ok: 'GameDeleted' | 'GameWasCancelled' | 'gameFinished' } & { Err: unknown };
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
  PlayerType,
  Step,
  MessagePayload,
  MessageHandlePayload,
  MessageDetails,
  Players,
  Properties,
  CellValues,
  PlayersByStrategyAddress,
};
