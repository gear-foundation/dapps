import { createContext, Dispatch, SetStateAction, useContext } from 'react';

import type { StoreItemsNames } from '@/app/types/ft-store';
import type { TamagotchiState } from '@/app/types/lessons';

export type TamagotchiContextValue = {
  tamagotchi: TamagotchiState | undefined;
  setTamagotchi: Dispatch<SetStateAction<TamagotchiState | undefined>>;
  tamagotchiItems: StoreItemsNames[];
  setTamagotchiItems: Dispatch<SetStateAction<StoreItemsNames[]>>;
};

export const TamagotchiCtx = createContext<TamagotchiContextValue | undefined>(undefined);

export function useTamagotchi() {
  const context = useContext(TamagotchiCtx);

  if (!context) {
    throw new Error('useTamagotchi must be used within a TmgProvider');
  }

  return context;
}
