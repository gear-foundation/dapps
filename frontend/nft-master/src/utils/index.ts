import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { LOCAL_STORAGE } from 'consts';

const copyToClipBoard = (value: string) => navigator.clipboard.writeText(value).then(() => console.log('Copied!'));

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

export { isLoggedIn, copyToClipBoard };
