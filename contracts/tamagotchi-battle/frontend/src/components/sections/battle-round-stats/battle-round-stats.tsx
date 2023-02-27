import clsx from 'clsx';
import { BattleRoundStatsAvatar } from 'components/sections/battle-round-stats-avatar';
import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { Countdown } from './counter';
import { motion } from 'framer-motion';

export const BattleRoundStats = () => {
  const { rivals, currentPlayer, battle, currentPairIdx } = useBattle();
  return (
    <div className="flex gap-10 justify-between items-center">
      {battle && (
        <>
          <BattleRoundStatsAvatar tamagotchi={rivals[0]} />
          {battle.state === 'GameIsOn' && !battle.pairs[currentPairIdx].gameIsOver && (
            <motion.div
              className="relative shrink-0"
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: 20 }}
              transition={{ duration: 0.5 }}>
              <BattleTurnArrows isReverse={rivals[1].tmgId === currentPlayer} />
              {battle && battle.pairs[currentPairIdx].moveDeadline && (
                <div className="absolute top-full left-1/2 -translate-x-1/2 flex flex-col mt-1.5 whitespace-nowrap">
                  <Countdown />
                </div>
              )}
            </motion.div>
          )}
          <BattleRoundStatsAvatar tamagotchi={rivals[1]} isReverse />
        </>
      )}
    </div>
  );
};

const BattleTurnArrows = ({ isReverse }: { isReverse: boolean }) => {
  const cn = 'w-7.5 xxl:w-10 aspect-[1/2] text-white';
  return (
    <div className={clsx('relative flex', isReverse && 'rotate-180')}>
      <Icon name="battle-next-step" className={clsx(cn, 'animate-battle-turn-1')} />
      <Icon name="battle-next-step" className={clsx(cn, 'animate-battle-turn-2')} />
      <Icon name="battle-next-step" className={clsx(cn, 'animate-battle-turn-3')} />
    </div>
  );
};
