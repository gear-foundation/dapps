import dayjs, { Dayjs } from 'dayjs';
import duration from 'dayjs/plugin/duration';
import { useEffect, useState } from 'react';

dayjs.extend(duration);

interface CountdownProps {
  endTime: Dayjs;
}

export const Countdown = ({ endTime }: CountdownProps) => {
  const [time, setTime] = useState<string>('00');

  useEffect(() => {
    const ms = 1000;
    const now = dayjs();
    const delta = endTime.unix() - now.unix();
    const diff = dayjs.duration(delta * 1000, 'milliseconds');
    const twoDP = (n: number) => (n > 9 ? n : '0' + n);

    let interval: NodeJS.Timer | undefined;

    if (diff.seconds() > 0) {
      interval = setInterval(function () {
        const duration = dayjs.duration(diff.asMilliseconds() - ms, 'milliseconds');
        const sec = duration.seconds();

        if (sec <= 0) {
          clearInterval(interval);
          setTime(`00`);
        } else setTime(`${twoDP(sec)}`);
      }, ms);
    }

    return () => {
      setTime('00');
      interval && clearInterval(interval);
    };
  }, [endTime]);

  return (
    <span className="inline-flex gap-1 font-kanit font-medium text-[28px] xxl:text-[40px] leading-none text-white text-center">
      <span className="py-2 px-1 w-[42px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
        {time.split('')[0]}
      </span>
      <span className="py-2 px-1 w-[42px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
        {time.split('')[1]}
      </span>
    </span>
  );
};
