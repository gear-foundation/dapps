import { useCallback, useRef, useState } from 'react';
import { useRefDimensions } from 'app/hooks/use-ref-dimensions';
import { motion, useAnimation } from 'framer-motion';
import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { useBattle } from 'app/context';
import { BattleStatePlayer } from 'app/types/battles';
import { TamagotchiAvatar } from '../../common/tamagotchi-avatar';
import { ScrollArea } from 'components/ui/scroll-area';

export const BattleTableChampions = () => {
  const ref = useRef<HTMLDivElement>(null);
  const [w] = useRefDimensions(ref);
  const [isActive, setIsActive] = useState<boolean>(false);
  const controls = useAnimation();

  const transition = { type: 'spring', damping: 20, stiffness: 160 };

  const onClick = useCallback(async () => {
    setIsActive((prev) => !prev);
    await controls.start(isActive ? 'active' : 'inactive');
  }, [controls, isActive]);

  const panel = {
    active: {
      x: 0,
    },
    inactive: {
      x: -w + 22,
    },
  };

  return (
    <motion.div
      key="table-champions"
      className="absolute left-full top-1/2 z-20 flex w-full max-w-[400px]"
      animate={{
        opacity: 1,
        x: -41 - 20,
        y: '-50%',
      }}
      initial={{
        opacity: 0,
        x: 0,
        y: '-50%',
      }}
      transition={{ delay: 0.5 }}>
      <motion.div
        key="table-champions-panel"
        className="flex w-full overflow-hidden"
        animate={controls}
        variants={panel}
        transition={transition}>
        <button
          className="inline-flex self-start my-10 px-2.5 py-8 btn--error bg-tertiary rounded-l-[6px]"
          onClick={onClick}>
          <span className="flex items-center gap-2.5 vertical-lr -rotate-180">
            <Icon name="double-arrows" className={clsx('w-4 h-4 text-white', !isActive && 'rotate-180')} />
            <span className="font-kanit font-semibold uppercase tracking-[0.04em]">Show champions</span>
            <Icon name="double-arrows" className={clsx('w-4 h-4 text-white', !isActive && 'rotate-180')} />
          </span>
        </button>

        <section
          ref={ref}
          className="relative grow p-8 px-6 bg-[#1D1D1D] border-2 border-r-transparent border-tertiary rounded-l-[20px] shadow">
          <div className="relative space-y-4">
            <h2 className="text-[28px] leading-8 text-tertiary font-kanit font-semibold tracking-[0.02em]">
              Champions
            </h2>
            <div className="flex items-center justify-between gap-5 px-4 text-xs leading-6 font-kanit tracking-[0.08em] text-white/60 uppercase bg-white/5 rounded-[30px]">
              <span>Player</span>
              <span>Kills</span>
            </div>
            <div className="mt-2.5">
              <BattleTableList />
            </div>
          </div>

          <motion.div
            key="table-champions-bubble"
            className="absolute top-0 left-6 w-[min(100%,306px)] blur-lg"
            animate={controls}
            variants={{
              active: {
                opacity: 0,
              },
              inactive: {
                opacity: 1,
              },
            }}
            aria-hidden>
            <Icon name="decorative-bubble" className="w-full aspect-[306/48] text-tertiary opacity-80" />
          </motion.div>
        </section>
      </motion.div>
    </motion.div>
  );
};

const BattleTableList = () => {
  const { players } = useBattle();

  return (
    <ScrollArea className="max-h-80 pr-3 -mr-3">
      <ul className="leading-4 space-y-1.5">
        {players
          .sort((p, c) => c.victories - p.victories)
          .map((player, i) => (
            <li key={i}>
              <BattleTablePairsRow player={player} position={i} />
            </li>
          ))}
      </ul>
    </ScrollArea>
  );
};

const BattleTablePairsRow = ({ player, position }: { player: BattleStatePlayer; position: number }) => {
  return (
    <div
      className={clsx(
        'flex items-center gap-4 w-full py-1 px-4 rounded-[30px] overflow-hidden',
        player.victories > 0 && position < 3 ? 'bg-gradient-to-b from-tertiary to-transparent' : 'bg-white/10',
      )}>
      <Icon
        name={
          player.victories > 0
            ? position === 0
              ? 'wins'
              : position === 1
              ? 'sword-2'
              : position === 2
              ? 'sword-1'
              : 'sword-single'
            : 'sword-single'
        }
        className="w-5 h-5"
      />
      <div className="relative w-10 aspect-square rounded-full overflow-hidden ring-4 ring-opacity-10 bg-white ring-white">
        <TamagotchiAvatar
          className="w-20 aspect-square -left-1/2 pointer-events-none"
          age={player.dateOfBirth}
          color={player.color}
        />
      </div>
      <div className="flex items-center gap-3 tracking-[0.03em] font-medium">
        <span className="w-20 truncate">{player.name}</span>
      </div>
      <p className="ml-auto text-2xl leading-none font-kanit font-medium w-7 text-center">{player.victories}</p>
    </div>
  );
};
