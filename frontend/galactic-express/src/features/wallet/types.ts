import { WALLET } from './consts';

export type WalletId = keyof typeof WALLET;

export type WalletValue = {
  name: string;
  SVG: string;
};

export type WalletEntry = [WalletId, WalletValue];
