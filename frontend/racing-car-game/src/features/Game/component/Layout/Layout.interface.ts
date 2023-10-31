import { GearCoreMessageUserUserMessage, HexString } from '@gear-js/api';

export interface LayoutProps {
  currentTurn: string;
}

export type WinStatus = 'Win' | 'Lose' | 'Draw' | null;

export interface RepliesItem {
  auto: GearCoreMessageUserUserMessage | null;
  manual: GearCoreMessageUserUserMessage | null;
}

export type RepliesQueue = RepliesItem[];

export type UserMessage = GearCoreMessageUserUserMessage & {
  details?: MessageDetails;
};

export interface MessageDetails {
  to?: HexString;
  code?: { Error: any };
}
