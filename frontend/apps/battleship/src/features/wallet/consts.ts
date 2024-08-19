import { atom } from 'jotai';
import { EnkryptSVG, PolkadotSVG, SubWalletSVG, TalismanSVG, NovaSVG } from './assets';

export const VOUCHER_MIN_LIMIT = 3;

const isNovaWallet = window?.walletExtension?.isNovaWallet;

export const WALLET = {
  'polkadot-js': {
    name: isNovaWallet ? 'Nova Wallet' : 'Polkadot JS',
    SVG: isNovaWallet ? NovaSVG : PolkadotSVG,
  },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
};

export const WALLETS = Object.entries(WALLET) as Entries<typeof WALLET>;

export const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);

export const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string; existentialDeposit: string }>(
  undefined,
);
