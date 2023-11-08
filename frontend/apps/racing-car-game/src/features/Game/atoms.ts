import { HexString } from '@gear-js/api';
import { atom } from 'jotai';
import { DecodedReply } from '@/types';

export const REPLY_DATA_ATOM = atom<DecodedReply | null>(null);

export const IS_STATE_READ_ATOM = atom<boolean>(false);

export const CURRENT_SENT_MESSAGE_ID_ATOM = atom<HexString | null>(null);

export const IS_STARTING_NEW_GAME_ATOM = atom<boolean>(false);
