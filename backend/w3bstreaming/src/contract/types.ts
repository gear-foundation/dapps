import { HexString } from '@gear-js/api';

export interface SubscriptionUser {
  accountId: string;
  subDate: string;
  nextWriteOff: string;
}

export interface User {
  name: string;
  surname: string;
  imgLink: string;
  streamIds: [string[]];
  subscribers: string[];
  subscriptions: SubscriptionUser[];
  role: string;
}

export interface UsersState {
  Users: [HexString, User][];
}
