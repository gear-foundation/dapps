import { TamagotchiBattleInfoCard } from '../components/cards/tamagotchi-battle-info-card';
import { Icon } from '../components/ui/icon';
import { useState } from 'react';
import { HexString } from '@polkadot/util/types';
import { PlayerColor } from '../app/types/battles';
import { AnimatePresence, motion } from 'framer-motion';
import { BattleRoundStatsAvatar } from '../components/sections/battle-round-stats-avatar';
import { Countdown } from '../components/sections/battle-round-stats/counter';
import clsx from 'clsx';

const rivals = [
  {
    color: 'Yellow' as PlayerColor,
    dateOfBirth: new Date().getMilliseconds(),
    defence: 2500,
    health: 2500,
    name: 'John',
    owner: '0x255' as HexString,
    power: 7500,
    tmgId: '0x123' as HexString,
    victories: 4,
  },
  {
    color: 'Green' as PlayerColor,
    dateOfBirth: new Date(new Date().getHours() + 1).getMilliseconds(),
    defence: 2500,
    health: 1000,
    name: 'Alex',
    owner: '0x2556' as HexString,
    power: 7500,
    tmgId: '0x1234' as HexString,
    victories: 7,
  },
];

export const Test = () => {
  const [active, setActive] = useState(false);
  const [show, setShow] = useState(false);

  return (
    <div className="container flex flex-col grow">
      <button onClick={() => setShow((prev) => !prev)}>Show tmg</button>

      <AnimatePresence>
        {show && (
          <>
            {/*Current Pair Stats*/}
            <div className="flex gap-10 justify-between items-center">
              <BattleRoundStatsAvatar tamagotchi={rivals[0]} />
              <motion.div
                className="relative shrink-0"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 20 }}
                transition={{ duration: 0.5 }}>
                <BattleTurnArrows isReverse={active} />
                <div className="absolute top-full left-1/2 -translate-x-1/2 flex flex-col mt-1.5 whitespace-nowrap">
                  <Countdown />
                </div>
              </motion.div>
              <BattleRoundStatsAvatar tamagotchi={rivals[1]} isReverse />
            </div>
            {/*Current Pair Battle*/}
            <div className="grow"></div>
            {/*Current Pair Cards*/}
            <div className="relative flex gap-10 justify-between mt-4 xxl:mt-7">
              <motion.div
                key="test-info-1"
                className="basis-[40%] flex justify-center"
                initial={{ opacity: 0, y: 120 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 120 }}
                transition={{ duration: 0.5 }}>
                <TamagotchiBattleInfoCard tamagotchi={rivals[0]} isActive={!active} />
              </motion.div>
              <motion.div
                key="test-info-stat"
                className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.5 }}>
                <div className="border border-white/10 bg-white/[3%] backdrop-blur-md p-6 pt-5 rounded-2xl font-kanit text-base text-white/60 tracking-wider">
                  <h3 className="font-normal text-center">
                    Participants: <b className="inline-block ml-1 text-xl font-semibold text-white">{43}</b>
                  </h3>
                  <div className="flex items-center gap-12 mt-4">
                    <div className="flex items-center gap-2">
                      <Icon name="participants-alive" className="w-6 h-6 shrink-0" />
                      <p className="flex items-center">
                        Alive: <b className="inline-block ml-1 text-xl font-semibold text-white">10</b>
                      </p>
                    </div>
                    <div className="flex items-center gap-2">
                      <Icon name="participants-dead" className="w-6 h-6 shrink-0" />
                      <p className="flex items-center">
                        Dead: <b className="inline-block ml-1 text-xl font-semibold text-white">33</b>
                      </p>
                    </div>
                  </div>
                </div>
              </motion.div>
              <motion.div
                key="test-info-2"
                className="basis-[40%] flex justify-center"
                initial={{ opacity: 0, y: 120 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 120 }}
                transition={{ duration: 0.5 }}>
                <TamagotchiBattleInfoCard tamagotchi={rivals[1]} isActive={active} />
              </motion.div>
            </div>
          </>
        )}
      </AnimatePresence>
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
