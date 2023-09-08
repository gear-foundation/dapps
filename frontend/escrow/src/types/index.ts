import { HexString } from '@polkadot/util/types';

type CreateFormValues = { buyer: string; seller: string; amount: string };

type Escrow = {
  buyer: HexString;
  seller: HexString;
  state: string;
  amount: string;
};

type Wallet = [string, Escrow];

export type { CreateFormValues, Escrow, Wallet };
