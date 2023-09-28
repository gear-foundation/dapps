import { atom } from 'jotai';
import { ADDRESS } from '@/consts';
import { Streams } from './features/StreamTeasers/types';
import { UsersRes } from './features/Account/types';

export const CONTRACT_ADDRESS_ATOM = atom(ADDRESS.CONTRACT);

export const STREAM_TEASERS_ATOM = atom<Streams | null>(null);

export const USERS_ATOM = atom<UsersRes | null>(null);
