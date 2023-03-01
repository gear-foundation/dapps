import { TamagotchiBattleInfoCard } from 'components/cards/tamagotchi-battle-info-card';
import { useBattle } from 'app/context';
import { Icon } from 'components/ui/icon';

import { AnimatePresence, motion } from 'framer-motion';

export const BattleRoundInfo = () => {
  const { rivals, currentPlayer, currentPairIdx, battle } = useBattle();
  return (
    <AnimatePresence key="round-info">
      <div className="relative flex gap-10 justify-between mt-4 xxl:mt-7">
        <motion.div
          key="player-info-1"
          className="basis-[40%] flex justify-center"
          initial={{ opacity: 0, y: 120 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: 120 }}
          transition={{ duration: 0.5 }}>
          <TamagotchiBattleInfoCard
            tamagotchi={rivals[0]}
            isActive={
              battle?.pairs[currentPairIdx].gameIsOver
                ? rivals[0].tmgId === battle?.pairs[currentPairIdx].winner
                : rivals[0].tmgId === currentPlayer
            }
          />
        </motion.div>
        <motion.div
          key="stats-info"
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.5 }}>
          <BattleRoundInfoBanner />
        </motion.div>
        <motion.div
          key="player-info-2"
          className="basis-[40%] flex justify-center"
          initial={{ opacity: 0, y: 120 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: 120 }}
          transition={{ duration: 0.5 }}>
          <TamagotchiBattleInfoCard
            tamagotchi={rivals[1]}
            isActive={
              battle?.pairs[currentPairIdx].gameIsOver
                ? rivals[1].tmgId === battle?.pairs[currentPairIdx].winner
                : rivals[1].tmgId === currentPlayer
            }
          />
        </motion.div>
      </div>
    </AnimatePresence>
  );
};

const BattleRoundInfoBanner = () => {
  const { players } = useBattle();

  return (
    <div className="border border-white/10 bg-white/[3%] backdrop-blur-md p-6 pt-5 rounded-2xl font-kanit text-base text-white/60 tracking-wider">
      <h3 className="font-normal text-center">
        Participants: <b className="inline-block ml-1 text-xl font-semibold text-white">{players.length}</b>
      </h3>
      <div className="flex items-center gap-12 mt-4">
        <div className="flex items-center gap-2">
          <Icon name="participants-alive" className="w-6 h-6 shrink-0" />
          <p className="flex items-center">
            Alive:{' '}
            <b className="inline-block ml-1 text-xl font-semibold text-white">
              {players.filter((el) => el.health).length}
            </b>
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Icon name="participants-dead" className="w-6 h-6 shrink-0" />
          <p className="flex items-center">
            Dead:{' '}
            <b className="inline-block ml-1 text-xl font-semibold text-white">
              {players.filter((el) => !el.health).length}
            </b>
          </p>
        </div>
      </div>
    </div>
  );
};
