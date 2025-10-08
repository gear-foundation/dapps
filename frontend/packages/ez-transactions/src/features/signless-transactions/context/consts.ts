const SIGNLESS_STORAGE_KEY = 'signless';

const DEFAULT_SIGNLESS_CONTEXT = {
  pair: undefined,
  storagePair: undefined,
  savePair: () => {},
  deletePair: () => {},
  unlockPair: () => {
    throw new Error('Context not initialized');
  },
  session: undefined,
  isSessionReady: false,
  isVoucherExists: false,
  voucherBalance: 0,
  createSession: () => Promise.resolve(),
  deleteSession: () => Promise.resolve(),
  voucher: undefined,
  storageVoucher: undefined,
  storageVoucherBalance: 0,
  isLoading: false,
  setIsLoading: () => {},
  isActive: false,
  isSessionActive: false,
  allowedActions: [],
  openSessionModal: async () => {},
  isAutoSignlessEnabled: false,
};

export { SIGNLESS_STORAGE_KEY, DEFAULT_SIGNLESS_CONTEXT };
