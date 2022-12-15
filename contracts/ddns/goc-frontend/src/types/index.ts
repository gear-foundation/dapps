import { Hex } from '@gear-js/api';

type Lottery = {
  admin: Hex;
  started: string;
  ending: string;
  participationCost: string;
  prizeFund: string;
  players: Hex[];
  winner: Hex;
  ftActorId: Hex;
};

type DashboardProps = {
  startTime: string;
  endTime: string;
  status: string;
  winner: Hex;
  countdown: string;
};

export type { Lottery, DashboardProps };
