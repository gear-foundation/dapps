const SIGNLESS_STORAGE_KEY = 'signless';

const DEFAULT_VALUES = {
  pair: undefined,
  savePair: () => {},
  deletePair: () => {},
  unlockPair: () => {},
  session: undefined,
  isSessionReady: false,
};

export { SIGNLESS_STORAGE_KEY, DEFAULT_VALUES };
