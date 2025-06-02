import { useAccount } from '@gear-js/react-hooks';
import { createContext, ReactNode, useContext, useEffect, useState } from 'react';

import type { StoreItemsNames } from '@/app/types/ft-store';
import type { TamagotchiState } from '@/app/types/lessons';

export const TamagotchiCtx = createContext({} as ReturnType<typeof useProgram>);

const useProgram = () => {
  const { account, isAccountReady } = useAccount();

  const [tamagotchi, setTamagotchi] = useState<TamagotchiState>();
  const [tamagotchiItems, setTamagotchiItems] = useState<StoreItemsNames[]>([]);

  useEffect(() => {
    if (!isAccountReady) return;

    setTamagotchi(undefined);
  }, [account, isAccountReady]);

  return {
    tamagotchi,
    setTamagotchi,
    tamagotchiItems,
    setTamagotchiItems,
  };
};

export const useTamagotchi = () => useContext(TamagotchiCtx);

export function TmgProvider({ children }: { children: ReactNode }) {
  const { Provider } = TamagotchiCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
