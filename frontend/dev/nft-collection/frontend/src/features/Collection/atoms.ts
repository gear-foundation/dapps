import { atom } from 'jotai';
import { OwnerToCollection } from '@/types';
import { Collections } from './types';

export const COLLECTION_CONTRACTS = atom<OwnerToCollection>([]);

export const COLLECTIONS = atom<Collections>({});
