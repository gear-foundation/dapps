import { HexString } from '@gear-js/api';
import { WALLET } from './consts';

type WalletId = keyof typeof WALLET;

type SystemAccount = {
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

export type { WalletId, SystemAccount };
