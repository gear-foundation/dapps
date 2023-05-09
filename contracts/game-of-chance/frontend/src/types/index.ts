import { HexString } from '@polkadot/util/types';

type Lottery = {
  admin: HexString;
  started: number;
  ending: number;
  participationCost: number;
  prizeFund: number;
  players: HexString[];
  winner: HexString;
  fungibleToken: HexString | null;
};

type DashboardProps = {
  startTime: string;
  endTime: string;
  status: string;
  winner: HexString;
  countdown: string;
};

export type { Lottery, DashboardProps };
