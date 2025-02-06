import { useCountdown } from '@dapps-frontend/hooks';

import { toNumber } from '@/app/utils';

import { useBattle } from '../../context';
import { getDoubleDigitSeconds } from '../../utils';

export const Countdown = () => {
  const { battle, currentPairIdx } = useBattle();

  const { moveDeadline } = battle?.pairs[currentPairIdx] || {};

  const msLeft = useCountdown(moveDeadline ? toNumber(moveDeadline) : undefined);
  const secondsLeft = msLeft !== undefined ? Math.floor(msLeft / 1000) : undefined;

  const formattedSeconds = secondsLeft !== undefined ? getDoubleDigitSeconds(secondsLeft) : undefined;
  const digits = formattedSeconds?.split('');

  return (
    <p className="flex flex-col items-center gap-1.5 text-center">
      <span className="font-semibold uppercase text-center text-[#D2D2D3] text-opacity-60 tracking-[.04em]">
        <span className="smh:hidden">Time left</span>

        <span className="smh:inline-block hidden">
          Round: {battle && +battle.pairs[currentPairIdx].rounds + 1} <span className="normal-case">of</span> 5
        </span>
      </span>

      {digits && (
        <span className="inline-flex gap-1 font-kanit font-medium text-[28px] xxl:text-[40px] leading-none text-white text-center">
          <span className="py-2 px-1 w-[40px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
            {digits[0]}
          </span>
          <span className="py-2 px-1 w-[40px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
            {digits[1]}
          </span>
        </span>
      )}
    </p>
  );
};
