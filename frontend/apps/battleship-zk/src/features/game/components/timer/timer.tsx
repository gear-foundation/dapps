import { useEffect, useState } from 'react';
import { getFormattedTime } from '../../utils';

type Props = {
  shouldGoOn: boolean;
  start_time: number | string | bigint | undefined;
};

export function Timer({ shouldGoOn, start_time }: Props) {
  const [elapsedTime, setElapsedTime] = useState('');

  useEffect(() => {
    if (shouldGoOn && start_time) {
      const updateTimer = () => {
        const currentTime = new Date().getTime();
        const startTime = Number(start_time);
        const elapsedTimeMilliseconds = currentTime - startTime;

        const formattedTime = getFormattedTime(elapsedTimeMilliseconds);

        shouldGoOn && setElapsedTime(formattedTime);
      };

      const timerInterval = setInterval(updateTimer, 1000);

      return () => {
        clearInterval(timerInterval);
      };
    }
  }, [shouldGoOn && start_time]);

  return <span>{elapsedTime}</span>;
}
