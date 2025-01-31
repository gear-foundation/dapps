import { atom } from 'jotai';
import { RegistrationStatus } from '@/features/session/types';
import { HexString } from '@gear-js/api';

export const CURRENT_GAME_ATOM = atom<HexString | null>(null);

export const PLAYER_NAME_ATOM = atom<string | null>(null);

export const REGISTRATION_STATUS = atom<RegistrationStatus>('registration');

export const IS_LOADING = atom<boolean>(false);
