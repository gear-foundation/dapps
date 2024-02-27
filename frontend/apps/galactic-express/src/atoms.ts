import { atom } from 'jotai';
import { RegistrationStatus } from 'features/session/types';

export const CURRENT_GAME_ATOM = atom<string>('');

export const PLAYER_NAME_ATOM = atom<string | null>(null);

export const REGISTRATION_STATUS = atom<RegistrationStatus>('registration');

export const IS_LOADING = atom<boolean>(false);
