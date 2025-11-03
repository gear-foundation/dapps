import { createContext } from 'react';

import type { GameContextValue } from './game-state';

const GameCtx = createContext<GameContextValue | undefined>(undefined);

export { GameCtx };
