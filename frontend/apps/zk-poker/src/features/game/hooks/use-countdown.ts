import { useEffect, useMemo, useState } from 'react';

import { useLobbyGameStartTimeQuery } from '../sails';

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

  const startTime = useMemo(() => toNumber(lobbyGameStartTime), [lobbyGameStartTime]);
  const limitMs = useMemo(() => toNumber(timeLimitMs), [timeLimitMs]);

  const [now, setNow] = useState(() => Date.now());

  useEffect(() => {
    const id = window.setInterval(() => setNow(Date.now()), 1000);
    const onVis = () => {
      if (!document.hidden) setNow(Date.now());
    };
    document.addEventListener('visibilitychange', onVis);
    return () => {
      window.clearInterval(id);
      document.removeEventListener('visibilitychange', onVis);
    };
  }, []);

  if (!limitMs || !startTime) {
    return { remainingMs: null, isRunning: false, isExpired: false };
  }

  const endTime = startTime + limitMs;
  const remainingMs = Math.max(0, endTime - now);

  return {
    remainingMs,
    isRunning: remainingMs > 0,
    isExpired: remainingMs === 0,
  };
}
