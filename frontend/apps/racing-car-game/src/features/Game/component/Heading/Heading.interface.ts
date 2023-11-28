import { WinStatus } from '../Layout/Layout.interface';

export interface HeadingProps {
  currentTurn: string;
  isPlayerAction: boolean;
  winStatus: WinStatus;
}
