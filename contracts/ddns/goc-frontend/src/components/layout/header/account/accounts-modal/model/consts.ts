import { PolkadotSVG, SubwalletSVG, TalismanSVG } from 'components/layout/icons';

const WALLET = {
  'polkadot-js': { name: 'Polkadot JS', icon: PolkadotSVG },
  'subwallet-js': { name: 'Subwallet', icon: SubwalletSVG },
  talisman: { name: 'Talisman', icon: TalismanSVG },
};

const WALLETS = Object.keys(WALLET);

export { WALLET, WALLETS };
