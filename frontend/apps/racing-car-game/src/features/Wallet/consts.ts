import { atom } from 'jotai';
import EnkryptSVG from '@/assets/icons/enkrypt-icon.svg';
import PolkadotSVG from '@/assets/icons/polkadot-js-icon.svg';
import SubWalletSVG from '@/assets/icons/sub-wallet-icon.svg';
import TalismanSVG from '@/assets/icons/talisman-icon.svg';
import NovaSVG from '@/assets/icons/nova.svg';
import { WalletValue } from './types';

interface InjectedWindow extends Window {
  walletExtension?: { isNovaWallet: boolean };
}

export const isNovaWallet = !!(window as InjectedWindow)?.walletExtension?.isNovaWallet;

export const WALLET = isNovaWallet
  ? {
      'polkadot-js': { name: 'Nova Wallet', SVG: NovaSVG },
      'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
    }
  : {
      'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
      'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
      talisman: { name: 'Talisman', SVG: TalismanSVG },
      enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
    };

export type WalletId = keyof typeof WALLET;

export type Wallets = [WalletId, WalletValue][];

export const WALLETS = Object.entries(WALLET) as Wallets;

export const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);
export const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string; existentialDeposit: string }>(
  undefined,
);
