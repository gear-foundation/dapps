import dayjs, { Dayjs } from 'dayjs';
import duration from 'dayjs/plugin/duration';
import { useEffect, useRef, useState } from 'react';
import { useBattle } from 'app/context';
import { toSeconds } from 'app/utils';

dayjs.extend(duration);

export const Countdown = () => {
  const { battle, currentPairIdx } = useBattle();
  const { time, isActive } = usePairCountdown();

  return (
    <>
      {isActive && (
        <p className="flex flex-col items-center gap-1.5 text-center">
          <span className="font-semibold uppercase text-center text-[#D2D2D3] text-opacity-60 tracking-[.04em]">
            <span className="smh:hidden">Time left</span>

            <span className="smh:inline-block hidden">
              Round: {battle && battle.pairs[currentPairIdx].rounds + 1} <span className="normal-case">of</span> 5
            </span>
          </span>

          <span className="inline-flex gap-1 font-kanit font-medium text-[28px] xxl:text-[40px] leading-none text-white text-center">
            <span className="py-2 px-1 w-[40px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
              {time.split('')[0]}
            </span>
            <span className="py-2 px-1 w-[40px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
              {time.split('')[1]}
            </span>
          </span>
        </p>
      )}
    </>
  );
};

const usePairCountdown = () => {
  const [time, setTime] = useState<string>('59');
  const { battle, currentPairIdx } = useBattle();
  const timer = useRef<NodeJS.Timer | undefined>();
  const prevTime = useRef<Dayjs | undefined>();

  useEffect(() => {
    if (battle && battle.pairs[currentPairIdx].moveDeadline) {
      // console.log('mount and update data');
      const now = dayjs();
      const deadline = dayjs(battle.pairs[currentPairIdx].moveDeadline);
      // const deadline = now.add(60, 'seconds');

      // console.log({
      //   test: deadline.format('HH:mm:ss'),
      //   dif: dayjs.duration(dayjs(deadline).diff(now)).format('HH:mm:ss'),
      //   difvalue: dayjs.duration(dayjs(deadline).diff(now)).asMilliseconds(),
      //   deadline: deadline.millisecond(),
      //   prev: prevTime.current?.millisecond(),
      // });

      if (prevTime.current?.millisecond() !== deadline.millisecond()) {
        const getDiff = () => dayjs.duration(dayjs(deadline).diff(now));
        const d = getDiff();
        // console.log('time is not equal', d.asSeconds());
        // console.log({ good_diff: d.asSeconds() > 0, good_ref: !timer.current });

        if (d.asSeconds() > 0 && !timer.current) {
          // console.log('init timer');
          const timerHandler = () => {
            const getDiff = () => dayjs.duration(dayjs(deadline).diff(dayjs()));
            // console.log('timer counts');
            const d = getDiff();

            if (d.asSeconds() <= 0) {
              // console.log('clean timer');
              clearInterval(timer.current);
              setTime(`00`);
              return;
            }

            setTime(toSeconds(d.seconds()));
          };

          timer.current = setInterval(timerHandler, 1000);
        } else setTime(`00`);

        prevTime.current = deadline;
      }
    }

    return () => {
      // console.log('umount from dynamic');
      prevTime.current = undefined;
      if (timer.current) {
        // console.log('destroy timer dynamic');
        clearInterval(timer.current);
        timer.current = undefined;
        setTime('59');
      }
    };
  }, [battle, currentPairIdx]);

  // useEffect(() => {
  //   console.log({ time });
  // }, [time]);

  // useEffect(() => {
  //   return () => {
  //     console.log('umount static');
  //     if (timer.current) {
  //       console.log('destroy timer static');
  //       clearInterval(timer.current);
  //       timer.current = undefined;
  //       prevTime.current = undefined;
  //       setTime('59');
  //     }
  //   };
  // }, []);

  return { time, isActive: timer.current };
};
