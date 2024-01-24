import { HexString } from '@polkadot/util/types';
import { Stream, Streams } from './features/StreamTeasers/types';
import { User, UsersRes } from './features/Account/types';

export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

export type ArrayElement<ArrayType extends readonly unknown[]> = ArrayType extends readonly (infer ElementType)[]
  ? ElementType
  : never;

export type ValueOf<T> = T[keyof T];

export type Handler = (event: Event) => void;

export interface GlobalState {
  users: { [key: HexString]: User };
  strems: { [key: string]: Streams };
}

export interface ProgramState {
  streams: Streams;
  users: UsersRes;
}

export interface ProgramStateRes {
  state?: ProgramState;
  isStateRead: Boolean;
  error: string;
}

export interface UsersState {
  Users: [HexString, User][];
}

export interface StreamsState {
  Streams: [string, Stream][];
}
