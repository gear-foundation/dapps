import clsx from 'clsx';
import { BattleRoundStatsAvatar } from 'components/sections/battle-round-stats-avatar';
import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { Countdown } from './counter';
import dayjs from 'dayjs';

export const BattleRoundStats = () => {
  const { rivals, currentPlayer, battle, currentPairIdx } = useBattle();
  return (
    <div className="flex gap-10 justify-between items-center">
      {battle && (
        <>
          <BattleRoundStatsAvatar tamagotchi={rivals[0]} />
          {battle.state === 'GameIsOn' && (
            <div className="relative shrink-0">
              <BattleTurnArrows isReverse={rivals[1].tmgId === currentPlayer} />
              {battle && battle.pairs[currentPairIdx].rounds >= 0 && (
                <div className="absolute top-full left-1/2 -translate-x-1/2 flex flex-col mt-1.5 whitespace-nowrap">
                  <p className="flex flex-col gap-1.5 text-center">
                    <span className="font-semibold uppercase text-[#D2D2D3] text-opacity-60 tracking-[.04em]">
                      Time left
                    </span>
                    <Countdown endTime={dayjs(battle.pairs[currentPairIdx].moveDeadline)} />
                  </p>
                </div>
              )}
            </div>
          )}
          <BattleRoundStatsAvatar tamagotchi={rivals[1]} isReverse />
        </>
      )}
    </div>
  );
};

const BattleTurnArrows = ({ isReverse }: { isReverse: boolean }) => (
  <div className={clsx('relative flex', isReverse && 'rotate-180')}>
    <Icon
      name="battle-next-step"
      className="w-8 xxl:w-10 aspect-[1/2] text-white animate-battle-turn-1 transition-opacity"
    />
    <Icon
      name="battle-next-step"
      className="w-8 xxl:w-10 aspect-[1/2] text-white animate-battle-turn-2 transition-opacity"
    />
    <Icon
      name="battle-next-step"
      className="w-8 xxl:w-10 aspect-[1/2] text-white animate-battle-turn-3 transition-opacity"
    />
  </div>
);
