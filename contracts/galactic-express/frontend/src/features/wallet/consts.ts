import { Entries } from 'types';
import { EnkryptSVG, PolkadotSVG, SubWalletSVG, TalismanSVG } from './assets';

const WALLET = {
  'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
};

const WALLETS = Object.entries(WALLET) as Entries<typeof WALLET>;

export { WALLET, WALLETS };
