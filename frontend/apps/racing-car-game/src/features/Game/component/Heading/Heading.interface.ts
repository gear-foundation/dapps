import { GameResult } from '@/app/utils';

export interface HeadingProps {
  currentTurn: string;
  isPlayerAction: boolean;
  winStatus: GameResult | null;
}
