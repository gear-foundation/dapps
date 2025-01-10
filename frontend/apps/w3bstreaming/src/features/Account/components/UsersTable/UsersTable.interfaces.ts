import { TableRow } from '@/ui/Table/Table.interfaces';

export interface SubscriptionsData extends TableRow {
  id: string;
  Streamer: string;
  img: string | null;
  'Date of next write-off': string;
  'Subscription Date': string;
}

export interface SubscribersData extends TableRow {
  id: string;
  User: string;
}

export interface UsersTableProps {
  data: SubscriptionsData[] | SubscribersData[];
  columns: string[];
  searchParams: {
    column: string;
  };
  sortedColumns?: string[];
  name: string;
}

export interface EmptyTableContentProps {
  name: string;
}
