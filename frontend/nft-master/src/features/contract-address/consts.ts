import { HexString } from '@polkadot/util/types';
import { atom } from 'jotai';
import { ADDRESS, LOCAL_STORAGE } from 'consts';

const CONTRACT_ADDRESS_ATOM = atom(
  (localStorage[LOCAL_STORAGE.CONTRACT_ADDRESS] as HexString | null) || ADDRESS.DEFAULT_CONTRACT,
);

export { CONTRACT_ADDRESS_ATOM };
