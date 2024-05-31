import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { GAME_OVER } from '@/feature/game/consts';

const totalTimeGame = 10 * 10 * 1000;

export const GameTimer = () => {
  const [gameOver, setGameOver] = useAtom(GAME_OVER);
  const [remainingTime, setRemainingTime] = useState(totalTimeGame);

  const [startTime, setStartTime] = useState<number | null>(null);
  useEffect(() => {
    if (gameOver) return;

    const start = Date.now();
    setStartTime(start);

    const updateTimer = () => {
      const now = Date.now();
      const elapsedTime = now - (startTime || now);
      const newRemainingTime = remainingTime - elapsedTime;

      if (newRemainingTime > 0) {
        setRemainingTime(newRemainingTime);
      } else if (newRemainingTime <= 0) {
        setGameOver(true);
        setRemainingTime(totalTimeGame);
      }
    };

    const timerId = setInterval(updateTimer, 1000);

    return () => clearInterval(timerId);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [startTime, gameOver, remainingTime]);

  const formatterTime = () => {
    const minutes = Math.floor((remainingTime / (1000 * 60)) % 60);
    const seconds = Math.floor((remainingTime / 1000) % 60);

    return `${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
  };

  return <>{formatterTime()}</>;
};
