import { HexString } from '@gear-js/api';
import { periods } from 'consts';

const initialValues = { isRenewal: true, period: periods[0].value };

export type InitialValues = typeof initialValues;

export type State = {
  Subscribers: FullSubState;
};

export type FullSubState = {
  [key: HexString]: {
    isActive: boolean;
    startBlock: string;
    endBlock: string;
    period: string;
    renewalDate: string;
    renewalBlock: string;
    price: string;
    willRenew: boolean;
    subscriptionStart: [string, string];
    subscriptionEnd: [string, string];
  };
};

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
