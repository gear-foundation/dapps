import { Profile } from '@/app/utils';

export interface LayoutProps {
  isBroadcaster: boolean;
  broadcasterId: string;
  title: string;
  description?: string | null;
  startTime: Date;
  broadcasterInfo: Profile;
  isUserSubscribed: boolean;
  streamId?: string;
}
