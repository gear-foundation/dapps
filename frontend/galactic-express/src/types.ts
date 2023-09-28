import { HexString } from '@gear-js/api';

type Escrow = {
  buyer: HexString;
  seller: HexString;
  state: string;
  amount: string;
};

type Wallet = [string, Escrow];

export type { Escrow, Wallet };
