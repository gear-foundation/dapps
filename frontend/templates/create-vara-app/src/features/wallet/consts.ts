import EnkryptSVG from './assets/enkrypt.svg?react';
import NovaSVG from './assets/nova.svg?react';
import PolkadotSVG from './assets/polkadot.svg?react';
import SubWalletSVG from './assets/subwallet.svg?react';
import TalismanSVG from './assets/talisman.svg?react';
import { Wallets } from './types';

type InjectedWindow = typeof globalThis & { walletExtension?: { isNovaWallet: boolean } };
const isNovaWallet = Boolean((window as unknown as InjectedWindow)?.walletExtension?.isNovaWallet);

const WALLET = isNovaWallet
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

const WALLETS = Object.entries(WALLET) as Wallets;

const IS_MOBILE_DEVICE = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);

export { WALLET, WALLETS, isNovaWallet, IS_MOBILE_DEVICE };
