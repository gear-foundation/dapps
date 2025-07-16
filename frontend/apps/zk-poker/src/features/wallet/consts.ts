import WebApp from '@twa-dev/sdk';
import { atom } from 'jotai';

import { EnkryptSVG, PolkadotSVG, SubWalletSVG, TalismanSVG, NovaSVG, VaranSVG } from './assets';

export const VOUCHER_MIN_LIMIT = 3;

const isNovaWallet = window?.walletExtension?.isNovaWallet;

const isInTelegram = WebApp.platform !== 'unknown';

export const EXTENTION_WALLET = {
  'polkadot-js': {
    name: isNovaWallet ? 'Nova Wallet' : 'Polkadot JS',
    SVG: isNovaWallet ? NovaSVG : PolkadotSVG,
  },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
};

export const TELEGRAM_WALLET = {
  varan: { name: 'Varan', SVG: VaranSVG },
};

export const WALLET = { ...EXTENTION_WALLET, ...TELEGRAM_WALLET };

export const WALLETS = Object.entries(isInTelegram ? TELEGRAM_WALLET : EXTENTION_WALLET) as Entries<typeof WALLET>;

export const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);

export const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string; existentialDeposit: string }>(
  undefined,
);
