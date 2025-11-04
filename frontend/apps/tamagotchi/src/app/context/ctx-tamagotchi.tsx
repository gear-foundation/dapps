import { useAccount } from '@gear-js/react-hooks';
import { JSX, PropsWithChildren, useEffect, useMemo, useState } from 'react';

import { TamagotchiCtx, TamagotchiContextValue } from './ctx-tamagotchi.context';

export function TmgProvider({ children }: PropsWithChildren): JSX.Element {
  const { account, isAccountReady } = useAccount();

  const [tamagotchi, setTamagotchi] = useState<TamagotchiContextValue['tamagotchi']>();
  const [tamagotchiItems, setTamagotchiItems] = useState<TamagotchiContextValue['tamagotchiItems']>([]);

  useEffect(() => {
    if (!isAccountReady) return;

    setTamagotchi(undefined);
    setTamagotchiItems([]);
  }, [account, isAccountReady]);

  const value = useMemo<TamagotchiContextValue>(
    () => ({
      tamagotchi,
      setTamagotchi,
      tamagotchiItems,
      setTamagotchiItems,
    }),
    [tamagotchi, tamagotchiItems],
  );

  return <TamagotchiCtx.Provider value={value}>{children}</TamagotchiCtx.Provider>;
}
