import { ReactComponent as EnkryptSVG } from './assets/enkrypt.svg';
import { ReactComponent as PolkadotSVG } from './assets/polkadot.svg';
import { ReactComponent as SubWalletSVG } from './assets/subwallet.svg';
import { ReactComponent as TalismanSVG } from './assets/talisman.svg';
import { ReactComponent as NovaSVG } from './assets/nova.svg';
import { Wallets } from './types';

const isNovaWallet = !!window?.walletExtension?.isNovaWallet;

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

export { WALLET, WALLETS, isNovaWallet };
