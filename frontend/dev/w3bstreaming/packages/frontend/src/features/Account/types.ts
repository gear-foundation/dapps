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

export interface UsersRes {
  [key: string]: User;
}

export interface WithDataProps {
  columns: string[];
  searchParams: {
    column: string;
  };
  sortedColumns?: string[];
  name?: 'Subscriptions' | 'Subscribers';
}
