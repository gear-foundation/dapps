import { useAtom } from 'jotai';
import { useLocation } from 'react-router-dom';
import { gameModeAtom, pendingAtom } from './store';
import { ROUTES } from '@/app/consts';

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useIsLocationGamePage() {
  const { pathname } = useLocation();

  return pathname === ROUTES.GAME;
}

export function useGameMode() {
  const [gameMode, setGameMode] = useAtom(gameModeAtom);

  const resetGameMode = () => {
    setGameMode(null);
  };

  return { gameMode, setGameMode, resetGameMode };
}
