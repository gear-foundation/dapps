import { HexString } from '@gear-js/api';
import { WALLET } from './consts';

export type WalletId = keyof typeof WALLET;

export type WalletValue = {
  name: string;
  SVG: string;
};

export type WalletEntry = [WalletId, WalletValue];

export type SystemAccount = {
  consumers: number; // 0
  data: {
    feeFrozen: number | HexString; // 0
    free: number | HexString; // '0x...'
    miscFrozen: number | HexString; // 0
    reserved: number | HexString; //  8327965542000
  };
  nonce: number; // 94
  providers: number; // 1
  sufficients: number; // 0
};
