import { TamagotchiBattleInfoCard } from '../components/cards/tamagotchi-battle-info-card';
import { Icon } from '../components/ui/icon';
import { useState } from 'react';
import { HexString } from '@polkadot/util/types';
import { PlayerColor } from '../app/types/battles';
import { AnimatePresence, motion } from 'framer-motion';
import { BattleRoundStatsAvatar } from '../components/sections/battle-round-stats-avatar';
import clsx from 'clsx';
import { TamagotchiAvatar } from '../components/common/tamagotchi-avatar';
import { buttonStyles } from '@gear-js/ui';
import { BattlePlayersQueue } from '../components/sections/battle-players-queue';

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

const cnWrapper = 'relative flex flex-col';
const cnT = 'm-auto h-full w-full max-w-full';

export const Test = () => {
  const [active, setActive] = useState(false);
  const [show, setShow] = useState(false);

  return (
    <>
      <div className="container flex flex-col grow">
        <button className="fixed top-5 left-1/2" onClick={() => setShow((prev) => !prev)}>
          Show tmg
        </button>
        <div className="flex flex-col flex-1">
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
                      <p className="flex flex-col gap-1.5 text-center">
                        <span className="font-semibold uppercase text-[#D2D2D3] text-opacity-60 tracking-[.04em]">
                          Time left
                        </span>

                        <span className="inline-flex gap-1 font-kanit font-medium text-[28px] xxl:text-[40px] leading-none text-white text-center">
                          <span className="py-2 px-1 w-[42px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
                            0
                          </span>
                          <span className="py-2 px-1 w-[42px] xxl:w-[50px] backdrop-blur-sm rounded-lg bg-gradient-to-b from-white/15 to-transparent">
                            0
                          </span>
                        </span>
                      </p>
                    </div>
                  </motion.div>
                  <BattleRoundStatsAvatar tamagotchi={rivals[1]} isReverse />
                </div>
                {/*Current Pair Battle*/}
                <div className="grow flex flex-col">
                  <div className="relative grow grid grid-cols-[40%_40%] justify-between gap-10 mt-10 xxl:mt-15">
                    <div className={cnWrapper}>
                      <TamagotchiAvatar
                        color={rivals[0].color}
                        age={rivals[0].dateOfBirth}
                        className={cnT}
                        isActive={!active}
                        isWinner={false}
                        isDead={!rivals[0].health}
                        damage={10}
                        action={'Skipped'}
                        asPlayer
                      />
                    </div>
                    <motion.div
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      exit={{ opacity: 0, transition: { delay: 0 } }}
                      transition={{ duration: 0.5, delay: 0.5 }}
                      className="absolute top-1/2 left-1/2 z-1 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-3 xxl:gap-6 w-full max-w-[200px] xxl:max-w-[250px]">
                      <div className="flex flex-col items-center gap-1 xxl:gap-2">
                        {!false ? (
                          <>
                            <p className="font-semibold font-sans uppercase text-[#D2D2D3] text-opacity-60 text-center tracking-[.04em]">
                              Round: 1 <span className="normal-case">of</span> 5
                            </p>
                            <p className="text-2xl leading-tight xxl:typo-h2 truncate max-w-[13ch] font-bold">
                              {rivals[0].name}
                            </p>
                          </>
                        ) : (
                          <p className="text-center text-2xl leading-normal xxl:typo-h2 truncate max-w-[13ch] font-bold">
                            <span className="text-primary">Name</span>
                            <br />
                            Winner
                          </p>
                        )}
                      </div>
                      <div className="space-y-2 xxl:space-y-3">
                        {false && (
                          <button
                            className={clsx(
                              'btn items-center gap-2 w-full transition-colors',
                              buttonStyles.primary,
                              buttonStyles.button,
                            )}>
                            Start New Round
                          </button>
                        )}
                        {true && (
                          <>
                            <button
                              className={clsx(
                                'btn btn--error items-center gap-2 w-full bg-error text-white transition-colors',
                                buttonStyles.button,
                              )}>
                              <Icon name="swords" className="w-5 h-5" /> Attack
                            </button>
                            <button
                              className={clsx(
                                'btn items-center gap-2 w-full',
                                buttonStyles.secondary,
                                buttonStyles.button,
                              )}>
                              <Icon name="armor" className="w-5 h-5" /> Defence
                            </button>
                          </>
                        )}
                      </div>
                    </motion.div>
                    <div className={cnWrapper}>
                      <TamagotchiAvatar
                        color={rivals[1].color}
                        age={rivals[1].dateOfBirth}
                        className={cnT}
                        isActive={active}
                        isWinner={false}
                        isDead={!rivals[1].health}
                        damage={20}
                        action={'Skipped'}
                        reverse
                        asPlayer
                      />
                    </div>
                  </div>
                </div>
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
      </div>
      <BattlePlayersQueue />
    </>
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
