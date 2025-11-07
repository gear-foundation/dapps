import { ReactNode, useMemo, useState } from 'react';

import { AppContext, type AppContextValue } from './context';

type AppProviderProps = {
  children: ReactNode;
};

export function AppProvider({ children }: AppProviderProps) {
  const [isPending, setIsPending] = useState(false);

  const value = useMemo<AppContextValue>(() => ({ isPending, setIsPending }), [isPending]);

  return <AppContext.Provider value={value}>{children}</AppContext.Provider>;
}
