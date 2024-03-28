import { atom } from 'jotai';

export const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);
export const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string; existentialDeposit: string }>(
  undefined,
);
