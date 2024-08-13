import { HexString } from '@polkadot/util/types';

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const createTamagotchiInitial = {
  programId: '' as HexString,
  programId2: '' as HexString,
  currentStep: 1,
};

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
};

export const MULTIPLIER = {
  MILLISECONDS: 1000,
  SECONDS: 60,
  MINUTES: 60,
  HOURS: 24,
};
