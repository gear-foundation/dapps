import { JSX, PropsWithChildren, useMemo, useState } from 'react';

import { AppCtx, AppContextValue } from './ctx-app.context';

export function AppProvider({ children }: PropsWithChildren): JSX.Element {
  const [isPending, setIsPending] = useState(false);

  const value = useMemo<AppContextValue>(
    () => ({
      isPending,
      setIsPending,
    }),
    [isPending],
  );

  return <AppCtx.Provider value={value}>{children}</AppCtx.Provider>;
}
