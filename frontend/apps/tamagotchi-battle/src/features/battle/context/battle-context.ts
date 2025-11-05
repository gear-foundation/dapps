import { createContext, useContext } from 'react';

import type { BattleContextValue } from './use-battle-state';

const BattleContext = createContext<BattleContextValue | undefined>(undefined);

function useBattle() {
  const context = useContext(BattleContext);

  if (!context) {
    throw new Error('useBattle must be used within a BattleProvider');
  }

  return context;
}

export { BattleContext, useBattle };
