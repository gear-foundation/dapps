import { useAtom } from 'jotai';
import { useEffect, useRef, useState } from 'react';

import { GAME_OVER, MS_TIME_GAME_OVER } from '@/feature/game/consts';

type GameTimerProps = {
  isPause: boolean;
};

export const GameTimer = ({ isPause }: GameTimerProps) => {
  const [, setTimeGameOver] = useAtom(MS_TIME_GAME_OVER);

  const [, setGameOver] = useAtom(GAME_OVER);
  const [elapsedTime, setElapsedTime] = useState(0);

  const [timeLeft, setTimeLeft] = useState('');

  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    if (isPause) {
      return;
    }

    startTimeRef.current = Date.now();

    const totalGameTime = 10 * 60 * 1000;

    const updateTimer = () => {
      const now = Date.now();
      const startTime = startTimeRef.current ?? now;
      const elapsed = now - startTime;
      const remainingTime = totalGameTime - elapsed;

      setElapsedTime(elapsed);

      if (remainingTime <= 0) {
        setTimeLeft('00:00');
        setGameOver(true);
      } else {
        const minutes = Math.floor((remainingTime / (1000 * 60)) % 60);
        const seconds = Math.floor((remainingTime / 1000) % 60);
        setTimeLeft(`${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`);
      }
    };

    updateTimer();
    const timerId = window.setInterval(updateTimer, 1000);

    return () => window.clearInterval(timerId);
  }, [isPause, setGameOver]);

  useEffect(() => {
    if (isPause) {
      setTimeGameOver(elapsedTime);
    }
  }, [elapsedTime, isPause, setTimeGameOver]);

  return <>{timeLeft}</>;
};
