import { HexString } from '@polkadot/util/types';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'consts';

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

const getProgramId = () => (localStorage[LOCAL_STORAGE.PROGRAM] || '') as HexString;

export { isLoggedIn, getProgramId };
