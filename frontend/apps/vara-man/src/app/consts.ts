import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
};

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GAME: import.meta.env.VITE_CONTRACT_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
};

export type IRegisterForm = {
  wallet: HexString | '';
  nickname: string;
};
export const initialRegister: IRegisterForm = {
  wallet: '',
  nickname: '',
};

export const initialCreateTournament = {
  bid: 0,
  DifficultyLevel: '',
  TournamentName: '',
  YourName: '',
  TournamentDuration: '',
};

export const SIGNLESS_ALLOWED_ACTIONS = [
  'DeletePlayer',
  'StartTournament',
  'CancelTournament',
  'CancelRegister',
  'LeaveGame',
  'RegisterForTournament',
  'RecordTournamentResult',
  'FinishSingleGame',
  'CreateNewTournament',
];
