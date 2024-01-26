import { atom } from 'jotai';
import { FullSubState, State } from 'types';

const STATE_ATOM = atom<State | null>(null);

const IS_STATE_READ_ATOM = atom(false);

const IS_AVAILABLE_BALANCE_READY = atom<boolean>(false);

const AVAILABLE_BALANCE = atom<undefined | { value: string; unit: string; existentialDeposit: string }>(undefined);

export { STATE_ATOM, IS_STATE_READ_ATOM, IS_AVAILABLE_BALANCE_READY, AVAILABLE_BALANCE };
