import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { HexString } from '@polkadot/util/types';
import { LOCAL_STORAGE } from 'consts';
import { getStatus, getCountdown } from './status';

const isLoggedIn = ({ address }: InjectedAccountWithMeta) => localStorage[LOCAL_STORAGE.ACCOUNT] === address;

const isWinner = (value: HexString) => !value.startsWith('0x00');

const getDate = (value: number) => new Date(value).toLocaleString();

export { isLoggedIn, isWinner, getDate, getStatus, getCountdown };
