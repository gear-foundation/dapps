import { createContext } from 'react';

import type { AppContextValue } from './app-state';

const AppCtx = createContext<AppContextValue | undefined>(undefined);

export { AppCtx };
