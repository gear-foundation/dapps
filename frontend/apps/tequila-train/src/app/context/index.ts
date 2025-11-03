import { useContext } from 'react';

import { AppCtx } from './ctx-app';
import { GameCtx } from './ctx-game';

const ensureContext = <T>(context: T | undefined, contextName: string): T => {
  if (!context) throw new Error(`${contextName} must be used within its provider`);
  return context;
};

export const useGame = () => ensureContext(useContext(GameCtx), 'Game context');
export const useApp = () => ensureContext(useContext(AppCtx), 'App context');

export { GameCtx, GameProvider } from './ctx-game';
export { AppCtx, AppProvider } from './ctx-app';
