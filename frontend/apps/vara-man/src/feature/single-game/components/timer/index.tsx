import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { GAME_OVER } from '@/feature/game/consts';

type Props = {
  gameRestarted: boolean;
};

export const GameTimer = ({ gameRestarted }: Props) => {
  const [, setGameOver] = useAtom(GAME_OVER);
  const [timeLeft, setTimeLeft] = useState('');
  const [startTime, setStartTime] = useState<number | null>(null);

  useEffect(() => {
    const start = Date.now();
    setStartTime(start);

    const totalGameTime = 10 * 60 * 1000;

    const updateTimer = () => {
      const now = Date.now();
      const elapsedTime = now - (startTime || now);
      const remainingTime = totalGameTime - elapsedTime;

      if (remainingTime <= 0) {
        setTimeLeft('00:00');
        setGameOver(true);
      } else {
        const minutes = Math.floor((remainingTime / (1000 * 60)) % 60);
        const seconds = Math.floor((remainingTime / 1000) % 60);
        setTimeLeft(`${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`);
      }
    };

    const timerId = setInterval(updateTimer, 1000);

    return () => clearInterval(timerId);
  }, [setGameOver, startTime, gameRestarted]);

  return <>{timeLeft}</>;
};
