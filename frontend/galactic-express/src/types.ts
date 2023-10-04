import { HexString } from '@gear-js/api';

type Escrow = {
  buyer: HexString;
  seller: HexString;
  state: string;
  amount: string;
};

type Wallet = [string, Escrow];

export type { Escrow, Wallet };

export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];
