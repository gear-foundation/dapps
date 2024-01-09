import { Entries } from '@/types';

import { ReactComponent as EnkryptSVG } from './assets/enkrypt.svg';
import { ReactComponent as PolkadotSVG } from './assets/polkadot.svg';
import { ReactComponent as SubWalletSVG } from './assets/subwallet.svg';
import { ReactComponent as TalismanSVG } from './assets/talisman.svg';

const WALLET = {
  'polkadot-js': { name: 'Polkadot JS', SVG: PolkadotSVG },
  'subwallet-js': { name: 'SubWallet', SVG: SubWalletSVG },
  talisman: { name: 'Talisman', SVG: TalismanSVG },
  enkrypt: { name: 'Enkrypt', SVG: EnkryptSVG },
};

const WALLETS = Object.entries(WALLET) as Entries<typeof WALLET>;

export { WALLET, WALLETS };
