import type { JSX, PropsWithChildren } from 'react';

import { BattleContext } from './battle-context';
import { useBattleState } from './use-battle-state';

function BattleProvider({ children }: PropsWithChildren): JSX.Element {
  const value = useBattleState();

  return <BattleContext.Provider value={value}>{children}</BattleContext.Provider>;
}

export { BattleProvider };
