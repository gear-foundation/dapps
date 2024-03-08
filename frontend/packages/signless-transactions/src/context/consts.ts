const SIGNLESS_STORAGE_KEY = 'signless';

const DEFAULT_VALUES = {
  pair: undefined,
  storagePair: undefined,
  savePair: () => {},
  deletePair: () => {},
  unlockPair: () => {},
  session: undefined,
  isSessionReady: false,
  isVoucherExists: false,
  voucherBalance: 0,
  createSession: () => {},
  deleteSession: () => {},
  updateSession: () => {},
  pairVoucherId: undefined,
  isLoading: false,
  setIsLoading: () => {},
  isAvailable: false,
};

export { SIGNLESS_STORAGE_KEY, DEFAULT_VALUES };
