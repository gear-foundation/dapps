import dayjs from 'dayjs';
import duration from 'dayjs/plugin/duration';
import { useEffect, useRef, useState } from 'react';
import { useBattle } from 'app/context';

dayjs.extend(duration);

export const Countdown = () => {
  const [time, setTime] = useState<string>('00');
  const { battle, currentPairIdx } = useBattle();
  const timer = useRef<NodeJS.Timer | undefined>(undefined);
  const prevTime = useRef<number | undefined>(undefined);

  useEffect(() => {
    console.log({ time });
  }, [time]);

  useEffect(() => {
    if (battle) {
      const deadline = battle.pairs[currentPairIdx].moveDeadline;
      const ms = 1000;
      const getDiff = () => dayjs.duration(dayjs(deadline).diff(dayjs()));
      const toSeconds = (n: number) => {
        const N = Math.abs(n);
        return N < 10 ? `0${N}` : `${N}`;
      };

      if (prevTime.current !== deadline) {
        console.log('time is not equal');
        if (getDiff().seconds() > 0 && !timer.current) {
          timer.current = setInterval(function () {
            console.log('timer counts');
            const d = getDiff();

            if (d.asMilliseconds() <= 0) {
              clearInterval(timer.current);
              setTime(`00`);
              return;
            }

            console.log(d.seconds());
            setTime(toSeconds(d.seconds()));
          }, ms);
        } else setTime(`00`);

        prevTime.current = deadline;
      }
    }

    return () => {
      if (timer.current) {
        setTime('00');
        clearInterval(timer.current);
      }
    };
  }, [battle, currentPairIdx]);

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
