import type { PropsWithChildren } from 'react';

import { AppCtx } from './app-context';
import { useAppState } from './app-state';

export { AppCtx } from './app-context';

export function AppProvider({ children }: PropsWithChildren) {
  const { Provider } = AppCtx;
  return <Provider value={useAppState()}>{children}</Provider>;
}
