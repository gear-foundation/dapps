import { AnyJson } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { User } from '../Account/types';

export type StreamProps = Stream;

export interface Stream {
  broadcaster: HexString;
  startTime: string;
  endTime: string;
  title: string;
  imgLink?: string;
  description?: string;
  broadcasterInfo?: User;
  watchers: [];
  timeCreation: string;
}

export interface Streams {
  [key: string]: Stream;
}

export interface FormattedTeaser {
  id: string;
  broadcaster: HexString;
  startTime: string;
  endTime: string;
  title: string;
  description?: string;
  imgLink?: string;
  watchers: [];
  timeCreation: string;
}
