import { useContext } from 'react';
import { BattleCtx } from './ctx-battle';
import { AppCtx } from './ctx-app';

export const useBattle = () => useContext(BattleCtx);
export const useApp = () => useContext(AppCtx);

export { BattleCtx, BattleProvider } from './ctx-battle';
export { AppCtx, AppProvider } from './ctx-app';
