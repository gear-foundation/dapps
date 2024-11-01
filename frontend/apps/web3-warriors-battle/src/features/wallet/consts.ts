import { EnkryptSVG, NovaIcon, PolkadotSVG, SubWalletSVG, TalismanSVG } from './assets';
import { IWalletExtensionContent, IWalletId } from '@/features/wallet/types';

export const WALLET_ID_LOCAL_STORAGE_KEY = 'wallet';

export const isNovaWallet = !!window?.walletExtension?.isNovaWallet;

export const WALLET = isNovaWallet
  ? {
      'polkadot-js': { name: 'Nova Wallet', SVG: NovaIcon },
      'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
    }
  : {
      'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
      'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
      talisman: { name: 'Talisman', SVG: TalismanSVG },
      enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
    };

export type Wallets = [IWalletId, IWalletExtensionContent][];

export const WALLETS = Object.entries(WALLET) as Wallets;
