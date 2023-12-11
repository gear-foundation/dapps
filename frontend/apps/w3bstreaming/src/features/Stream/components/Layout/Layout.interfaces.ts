import { User } from '@/features/Account/types';

export interface LayoutProps {
  isBroadcaster: boolean;
  broadcasterId: string;
  title: string;
  description?: string;
  startTime: Date;
  broadcasterInfo: User;
  isUserSubscribed: boolean;
  streamId?: string;
}
