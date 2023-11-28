import { atom } from 'jotai';

export const CURRENT_CONTRACT_ADDRESS_ATOM = atom<string>('');

export const IS_CONTRACT_ADDRESS_INITIALIZED_ATOM = atom<boolean>(false);
