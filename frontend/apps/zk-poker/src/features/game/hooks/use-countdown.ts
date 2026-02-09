import { useEffect, useMemo, useRef, useState } from 'react';

import { useCurrentTimeQuery, useLobbyGameStartTimeQuery } from '../sails';

type CountdownState = {
  remainingMs: number | null;
  isRunning: boolean;
  isExpired: boolean;
};

const toNumber = (value: number | string | bigint | null | undefined): number | null => {
  if (value === null || value === undefined) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
};

export function useCountdown(timeLimitMs?: number | string | bigint | null): CountdownState {
  const { lobbyGameStartTime } = useLobbyGameStartTimeQuery();
  const { currentTime } = useCurrentTimeQuery();

  const startTime = useMemo(() => toNumber(lobbyGameStartTime), [lobbyGameStartTime]);
  const limitMs = useMemo(() => toNumber(timeLimitMs), [timeLimitMs]);

  const syncRef = useRef<{ serverNow: number; clientNow: number } | null>(null);
  const [, forceTick] = useState(0);

  useEffect(() => {
    const numericCurrent = toNumber(currentTime);
    if (numericCurrent === null) return;
    syncRef.current = { serverNow: numericCurrent, clientNow: Date.now() };
    forceTick((x) => x + 1);
  }, [currentTime]);

  useEffect(() => {
    const id = window.setInterval(() => forceTick((x) => x + 1), 1000);
    const onVis = () => {
      if (!document.hidden) forceTick((x) => x + 1);
    };
    document.addEventListener('visibilitychange', onVis);
    return () => {
      window.clearInterval(id);
      document.removeEventListener('visibilitychange', onVis);
    };
  }, []);

  if (!limitMs || !startTime || !syncRef.current) {
    return { remainingMs: null, isRunning: false, isExpired: false };
  }

  const now = syncRef.current.serverNow + (Date.now() - syncRef.current.clientNow);
  const endTime = startTime + limitMs;
  const remainingMs = Math.max(0, endTime - now);

  return {
    remainingMs,
    isRunning: remainingMs > 0,
    isExpired: remainingMs === 0,
  };
}
