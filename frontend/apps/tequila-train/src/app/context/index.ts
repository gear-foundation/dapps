import { useContext } from 'react';

import { AppCtx } from './ctx-app';
import { GameCtx } from './ctx-game';

export const useGame = () => useContext(GameCtx);
export const useApp = () => useContext(AppCtx);

export { GameCtx, GameProvider } from './ctx-game';
export { AppCtx, AppProvider } from './ctx-app';
