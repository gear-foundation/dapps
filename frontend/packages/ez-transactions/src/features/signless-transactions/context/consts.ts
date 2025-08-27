const SIGNLESS_STORAGE_KEY = 'signless';

const DEFAULT_SIGNLESS_CONTEXT = {
  pair: undefined,
  storagePair: undefined,
  savePair: () => {},
  deletePair: () => {},
  unlockPair: () => {
    throw new Error('unlockPair is not implemented');
  },
  session: undefined,
  isSessionReady: false,
  isVoucherExists: false,
  voucherBalance: 0,
  createSession: () => {},
  deleteSession: () => {},
  voucher: undefined,
  storageVoucher: undefined,
  storageVoucherBalance: 0,
  isLoading: false,
  setIsLoading: () => {},
  isActive: false,
  isSessionActive: false,
  allowedActions: [],
  voucherIssueAmount: 0,
  voucherReissueThreshold: 0,
};

export { SIGNLESS_STORAGE_KEY, DEFAULT_SIGNLESS_CONTEXT };
