import { BattleRoundStatsAvatar } from '../battle-round-stats-avatar';
import { SpriteIcon } from 'components/ui/sprite-icon';
import { useBattle } from '../../context';
import { Countdown } from './counter';
import { motion } from 'framer-motion';
import { cn } from 'app/utils';

export const BattleRoundStats = () => {
  const { rivals, currentPlayer, battle, currentPairIdx } = useBattle();

  console.log('----------STATS----------');
  console.log('battle');
  console.log(battle);
  console.log('currentPairIdx', currentPairIdx);
  console.log('battle?.pairs[currentPairIdx]', battle?.pairs[currentPairIdx]);
  console.log('state ---', battle?.state);
  console.log('---------------------------');

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
  const cx = 'smh:w-6 w-7.5 xxl:w-10 aspect-[1/2] text-white';
  return (
    <div className={cn('relative flex', isReverse && 'rotate-180')}>
      <SpriteIcon name="battle-next-step" className={cn(cx, 'animate-battle-turn-1')} />
      <SpriteIcon name="battle-next-step" className={cn(cx, 'animate-battle-turn-2')} />
      <SpriteIcon name="battle-next-step" className={cn(cx, 'animate-battle-turn-3')} />
    </div>
  );
};
