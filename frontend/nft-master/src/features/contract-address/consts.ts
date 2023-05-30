import { LOCAL_STORAGE } from 'consts';
import { atom } from 'jotai';

const CONTRACT_ADDRESS_ATOM = atom((localStorage[LOCAL_STORAGE.CONTRACT_ADDRESS] as string) || '');

export { CONTRACT_ADDRESS_ATOM };
