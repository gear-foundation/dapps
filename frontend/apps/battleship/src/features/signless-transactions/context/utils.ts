import { KeyringPair$Json } from '@polkadot/keyring/types';
import { Keyring } from '@polkadot/api';

import { LOCAL_STORAGE_SIGNLESS_PAIR_KEY } from './consts';

const getSavedPair = () => {
  const localStoragePairJson = localStorage[LOCAL_STORAGE_SIGNLESS_PAIR_KEY];
  if (!localStoragePairJson) return;

  const pairJson = JSON.parse(localStoragePairJson) as KeyringPair$Json;
  const keyring = new Keyring({ type: 'sr25519' });

  return keyring.addFromJson(pairJson);
};

export { getSavedPair };
