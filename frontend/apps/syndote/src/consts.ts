import AcalaSVG from '@/assets/images/logos/acala.svg?react';
import AstarSVG from '@/assets/images/logos/astar.svg?react';
import BifrostSVG from '@/assets/images/logos/bifrost.svg?react';
import centrifuge from '@/assets/images/logos/centrifuge.png';
import DockSVG from '@/assets/images/logos/dock.svg?react';
import EfinitySVG from '@/assets/images/logos/efinity.svg?react';
import ErrorSVG from '@/assets/images/logos/error.svg?react';
import FireSVG from '@/assets/images/logos/fire.svg?react';
import GasSVG from '@/assets/images/logos/gas.svg?react';
import GearSVG from '@/assets/images/logos/gear.svg?react';
import HackSVG from '@/assets/images/logos/hack.svg?react';
import hydradx from '@/assets/images/logos/hydradx.png';
import i from '@/assets/images/logos/i.png';
import InterlaySVG from '@/assets/images/logos/interlay.svg?react';
import KiltSVG from '@/assets/images/logos/kilt.svg?react';
import KusamaSVG from '@/assets/images/logos/kusama.svg?react';
import LitentrySVG from '@/assets/images/logos/litentry.svg?react';
import MSVG from '@/assets/images/logos/m.svg?react';
import MoonbeamSVG from '@/assets/images/logos/moonbeam.svg?react';
import NodleSVG from '@/assets/images/logos/nodle.svg?react';
import NovaSVG from '@/assets/images/logos/nova.svg?react';
import ParallelSVG from '@/assets/images/logos/parallel.svg?react';
import PhalaSVG from '@/assets/images/logos/phala.svg?react';
import PolkadotSVG from '@/assets/images/logos/polkadot.svg?react';
import PolkadotJsSVG from '@/assets/images/logos/polkadotjs.svg?react';
import RmrkSVG from '@/assets/images/logos/rmrk.svg?react';
import RobonomicsSVG from '@/assets/images/logos/robonomics.svg?react';
import SubquerySVG from '@/assets/images/logos/subquery.svg?react';
import SubsocialSVG from '@/assets/images/logos/subsocial.svg?react';
import subsquid from '@/assets/images/logos/subsquid.png';
import subwallet from '@/assets/images/logos/subwallet.png';
import TalismanSVG from '@/assets/images/logos/talisman.svg?react';
import unique from '@/assets/images/logos/unique.png';
import VaraSVG from '@/assets/images/logos/vara.svg?react';
import { PlayerType } from '@/types';

const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
};

enum LocalStorage {
  Account = 'account',
  Wallet = 'wallet',
  Player = 'player',
}

const fields = [
  { Image: GearSVG, type: 'none' },
  {
    Image: NodleSVG,
    type: 'cell',
    values: {
      heading: 'Nodle',
      baseRent: '100',
      bronze: '100',
      silver: '100',
      gold: '100',
      cell: '1000',
    },
  },
  {
    Image: FireSVG,
    type: 'none',
  },
  {
    Image: RobonomicsSVG,
    type: 'cell',
    values: {
      heading: 'Robonomics Network',
      baseRent: '105',
      bronze: '105',
      silver: '105',
      gold: '105',
      cell: '1050',
    },
  },
  { Image: ErrorSVG, type: 'none' },
  {
    Image: PolkadotJsSVG,
    type: 'cell',
    values: {
      heading: 'polkadot{.js}',
      baseRent: '110',
      bronze: '110',
      silver: '110',
      gold: '110',
      cell: '1100',
    },
  },
  {
    Image: MSVG,
    type: 'cell',
    values: {
      heading: 'GM',
      baseRent: '150',
      bronze: '150',
      silver: '150',
      gold: '150',
      cell: '1500',
    },
  },
  { Image: FireSVG, type: 'none' },
  {
    Image: i,
    type: 'cell',
    values: {
      heading: 'ChaosDAO',
      baseRent: '160',
      bronze: '160',
      silver: '160',
      gold: '160',
      cell: '1600',
    },
  },
  {
    Image: SubsocialSVG,
    type: 'cell',
    values: {
      heading: 'Subsocial',
      baseRent: '170',
      bronze: '170',
      silver: '170',
      gold: '170',
      cell: '1700',
    },
  },
  { Image: GasSVG, type: 'none' },
  {
    Image: InterlaySVG,
    type: 'cell',
    values: {
      heading: 'Interlay',
      baseRent: '200',
      bronze: '200',
      silver: '200',
      gold: '200',
      cell: '2000',
    },
  },
  {
    Image: SubquerySVG,
    type: 'cell',
    values: {
      heading: 'Subquery',
      baseRent: '205',
      bronze: '205',
      silver: '205',
      gold: '205',
      cell: '2050',
    },
  },
  {
    Image: BifrostSVG,
    type: 'cell',
    values: {
      heading: 'Bifrost',
      baseRent: '210',
      bronze: '210',
      silver: '210',
      gold: '210',
      cell: '2100',
    },
  },
  {
    Image: ParallelSVG,
    type: 'cell',
    values: {
      heading: 'Parallel',
      baseRent: '220',
      bronze: '220',
      silver: '220',
      gold: '220',
      cell: '2200',
    },
  },
  {
    Image: NovaSVG,
    type: 'cell',
    values: {
      heading: 'Nova',
      baseRent: '230',
      bronze: '230',
      silver: '230',
      gold: '230',
      cell: '2300',
    },
  },
  { Image: FireSVG, type: 'none' },
  {
    Image: LitentrySVG,
    type: 'cell',
    values: {
      heading: 'Litentry',
      baseRent: '240',
      bronze: '240',
      silver: '240',
      gold: '240',
      cell: '2400',
    },
  },
  {
    Image: DockSVG,
    type: 'cell',
    values: {
      heading: 'Dock',
      baseRent: '245',
      bronze: '245',
      silver: '245',
      gold: '245',
      cell: '2450',
    },
  },
  {
    Image: KiltSVG,
    type: 'cell',
    values: {
      heading: 'Kilt',
      baseRent: '250',
      bronze: '250',
      silver: '250',
      gold: '250',
      cell: '2500',
    },
  },
  { Image: VaraSVG, type: 'none' },
  {
    Image: unique,
    type: 'cell',
    values: {
      heading: 'Unique',
      baseRent: '300',
      bronze: '300',
      silver: '300',
      gold: '300',
      cell: '3000',
    },
  },
  { Image: FireSVG, type: 'none' },
  {
    Image: RmrkSVG,
    type: 'cell',
    values: {
      heading: 'RMRK',
      baseRent: '310',
      bronze: '310',
      silver: '310',
      gold: '310',
      cell: '3100',
    },
  },
  {
    Image: EfinitySVG,
    type: 'cell',
    values: {
      heading: 'Efinity',
      baseRent: '315',
      bronze: '315',
      silver: '315',
      gold: '315',
      cell: '3150',
    },
  },
  {
    Image: subwallet,
    type: 'cell',
    values: {
      heading: 'Subwallet',
      baseRent: '320',
      bronze: '320',
      silver: '320',
      gold: '320',
      cell: '3200',
    },
  },
  {
    Image: hydradx,
    type: 'cell',
    values: {
      heading: 'HydraDX',
      baseRent: '325',
      bronze: '325',
      silver: '325',
      gold: '325',
      cell: '3250',
    },
  },
  {
    Image: centrifuge,
    type: 'cell',
    values: {
      heading: 'Centrifuge',
      baseRent: '330',
      bronze: '330',
      silver: '330',
      gold: '330',
      cell: '3300',
    },
  },
  {
    Image: subsquid,
    type: 'cell',
    values: {
      heading: 'Subsquid',
      baseRent: '335',
      bronze: '335',
      silver: '335',
      gold: '335',
      cell: '3350',
    },
  },
  {
    Image: AcalaSVG,
    type: 'cell',
    values: {
      heading: 'Acala',
      baseRent: '340',
      bronze: '340',
      silver: '340',
      gold: '340',
      cell: '3400',
    },
  },
  { Image: HackSVG, type: 'none' },
  {
    Image: PhalaSVG,
    type: 'cell',
    values: {
      heading: 'Phala',
      baseRent: '400',
      bronze: '400',
      silver: '400',
      gold: '400',
      cell: '4000',
    },
  },
  {
    Image: AstarSVG,
    type: 'cell',
    values: {
      heading: 'Astar',
      baseRent: '405',
      bronze: '405',
      silver: '405',
      gold: '405',
      cell: '4050',
    },
  },
  { Image: FireSVG, type: 'none' },
  {
    Image: MoonbeamSVG,
    type: 'cell',
    values: {
      heading: 'Moonbeam',
      baseRent: '410',
      bronze: '410',
      silver: '410',
      gold: '410',
      cell: '4100',
    },
  },
  {
    Image: TalismanSVG,
    type: 'cell',
    values: {
      heading: 'Talisman',
      baseRent: '415',
      bronze: '415',
      silver: '415',
      gold: '415',
      cell: '4150',
    },
  },
  { Image: ErrorSVG, type: 'none' },
  {
    Image: KusamaSVG,
    type: 'cell',
    values: {
      heading: 'Kusama',
      baseRent: '420',
      bronze: '420',
      silver: '420',
      gold: '420',
      cell: '4200',
    },
  },
  { Image: FireSVG, type: 'none' },
  {
    Image: PolkadotSVG,
    type: 'cell',
    values: {
      heading: 'Polkadot',
      baseRent: '450',
      bronze: '450',
      silver: '450',
      gold: '450',
      cell: '4500',
    },
  },
];

const INIT_PLAYERS = [
  { color: 'pink' as PlayerType['color'] },
  { color: 'purple' as PlayerType['color'] },
  { color: 'yellow' as PlayerType['color'] },
  { color: 'green' as PlayerType['color'] },
];

export { ENV, LocalStorage, fields, INIT_PLAYERS };
