import { EnkryptSVG, PolkadotSVG, SubWalletSVG, TalismanSVG, NovaSVG } from './assets';

export const WALLET_ID_LOCAL_STORAGE_KEY = 'wallet';

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
