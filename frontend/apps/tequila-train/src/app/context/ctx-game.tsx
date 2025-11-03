import type { PropsWithChildren } from 'react';

import { GameCtx } from './game-context';
import { useGameState } from './game-state';

export { GameCtx } from './game-context';

export function GameProvider({ children }: PropsWithChildren) {
  const { Provider } = GameCtx;
  return <Provider value={useGameState()}>{children}</Provider>;
}
