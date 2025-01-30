import { useGame } from '@/app/context';
import { useEffect, useState } from 'react';

const Timer = () => {
  const { timer } = useGame();
  const [seconds, setSeconds] = useState(timer);

  useEffect(() => {
    setSeconds(timer);
  }, [timer]);

  useEffect(() => {
    const interval = setInterval(() => {
      setSeconds((prevSeconds) => {
        if (prevSeconds > 0) return prevSeconds - 1;
        clearInterval(interval);
        return 0;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [seconds]);

  return <h3 className="absolute  left-[80px] text-[70px] opacity-50 text-white">{seconds.toFixed(0)}</h3>;
};

export default Timer;
