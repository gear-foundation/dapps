const SIGNLESS_STORAGE_KEY = 'signless';

const DEFAULT_VALUES = {
  pair: undefined,
  storagePair: undefined,
  savePair: () => {},
  deletePair: () => {},
  unlockPair: () => {},
  session: undefined,
  isSessionReady: false,
  voucherBalance: 0,
};

export { SIGNLESS_STORAGE_KEY, DEFAULT_VALUES };
