import { useContext } from 'react';
import { GameCtx } from './ctx-game';
import { AppCtx } from './ctx-app';

export const useGame = () => useContext(GameCtx);
export const useApp = () => useContext(AppCtx);

export { GameCtx, GameProvider } from './ctx-game';
export { AppCtx, AppProvider } from './ctx-app';
