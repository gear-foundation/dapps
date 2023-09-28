import { atom } from 'jotai';
import EnkryptSVG from '@/assets/icons/enkrypt-icon.svg';
import PolkadotSVG from '@/assets/icons/polkadot-js-icon.svg';
import SubWalletSVG from '@/assets/icons/sub-wallet-icon.svg';
import TalismanSVG from '@/assets/icons/talisman-icon.svg';
import { Entries } from '@/types';

export const WALLET_ID_LOCAL_STORAGE_KEY = 'wallet';

export const WALLET = {
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
  'polkadot-js': {
    name: window?.walletExtension?.isNovaWallet ? 'Nova Wallet' : 'Polkadot JS',
    SVG: (window as Window)?.walletExtension?.isNovaWallet ? EnkryptSVG : PolkadotSVG,
  },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
};

export const WALLETS = Object.entries(WALLET) as Entries<typeof WALLET>;

export const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);
export const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string }>(undefined);
