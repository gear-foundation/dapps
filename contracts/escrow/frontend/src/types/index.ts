import { HexString } from '@polkadot/util/types';

type CreateFormValues = { buyer: string; seller: string; amount: string };

type Escrow = {
  buyer: HexString;
  seller: HexString;
  state: string;
  amount: number;
};

type Wallet = [number, Escrow];

export type { CreateFormValues, Escrow, Wallet };
