import { KeyringPair$Json } from '@polkadot/keyring/types';

import { SIGNLESS_STORAGE_KEY } from './consts';

const getStorage = () =>
  JSON.parse(localStorage.getItem(SIGNLESS_STORAGE_KEY) || '{}') as Record<string, KeyringPair$Json | undefined>;

export { getStorage };
