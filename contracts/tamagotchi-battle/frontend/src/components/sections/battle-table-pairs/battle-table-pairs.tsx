import { Icon } from 'components/ui/icon';
import { useBattle } from 'app/context';
import { useCallback, useEffect, useRef, useState } from 'react';
import type { BattleStatePair, BattleStatePlayer } from 'app/types/battles';
import clsx from 'clsx';
import { useAnimation, motion } from 'framer-motion';
import { useRefDimensions } from 'app/hooks/use-ref-dimensions';
import { nanoid } from 'nanoid';
import { ScrollArea } from '../../ui/scroll-area';

type PairData = { players: BattleStatePlayer[]; pair: BattleStatePair; id: string; idx: number };

export const BattleTablePairs = () => {
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
      x: w - 22,
    },
  };

  return (
    <motion.div
      key="table-pairs"
      className="absolute right-full top-1/2 z-20 flex"
      animate={{
        opacity: 1,
        x: 41 + 20,
        y: '-50%',
      }}
      initial={{
        opacity: 0,
        x: 0,
        y: '-50%',
      }}
      transition={{ delay: 0.5 }}>
      <motion.div
        key="table-pairs-panel"
        className="flex overflow-hidden"
        animate={controls}
        variants={panel}
        transition={transition}>
        <section
          ref={ref}
          className="relative p-8 px-6 bg-[#1D1D1D] border-2 border-l-transparent border-primary rounded-r-[20px] shadow">
          <div className="relative space-y-4">
            <h2 className="text-[28px] leading-8 text-primary font-kanit font-semibold tracking-[0.02em]">Battles</h2>
            <div className="flex items-center justify-between gap-5 px-4 text-xs leading-6 font-kanit tracking-[0.08em] text-white/60 uppercase bg-white/5 rounded-[30px]">
              <span>Battle</span>
              <span>Status</span>
            </div>
            <div className="mt-2.5">
              <BattleTableList />
            </div>
          </div>

          <div className="absolute top-0 left-6 w-[min(100%,306px)] blur-lg" aria-hidden>
            <Icon name="decorative-bubble" className="w-full aspect-[306/48] text-primary opacity-80" />
          </div>
        </section>

        <button
          className="inline-flex self-start my-10 px-2.5 py-8 btn--primary bg-primary rounded-r-[6px]"
          onClick={onClick}>
          <span className="flex items-center gap-2.5 vertical-rl -rotate-180">
            <Icon name="double-arrows" className={clsx('w-4 h-4 text-white', isActive && 'rotate-180')} />
            <span className="font-kanit font-semibold uppercase tracking-[0.04em]">Show battles</span>
            <Icon name="double-arrows" className={clsx('w-4 h-4 text-white', isActive && 'rotate-180')} />
          </span>
        </button>
      </motion.div>
    </motion.div>
  );
};

// drag="x"
// dragElastic={0.1}
// dragConstraints={{
//   left: -w + 20,
//     right: 0,
// }}
// dragMomentum={false}
// onDragEnd={async (_event, info) => {
//   const isDraggingLeft = info.offset.x < 0;
//   const multiplier = isDraggingLeft ? 1 / 4 : 2 / 3;
//   const threshold = width * multiplier;
//
//   if (Math.abs(info.point.x) > threshold && isActive) {
//     console.log('if');
//     setIsActive(false);
//   } else if (Math.abs(info.point.x) < threshold && !isActive) {
//     console.log('else 1');
//     setIsActive(true);
//   } else {
//     console.log('else');
//     await controls.start(isActive ? 'active' : 'inactive');
//   }
// }}

const BattleTableList = () => {
  const { battle, currentPairIdx } = useBattle();
  const [pairs, setPairs] = useState<PairData[]>([]);

  useEffect(() => {
    if (battle) {
      const _pairs = Object.values(battle.pairs);
      const final: PairData[] = [];

      _pairs.forEach((pair, i) => {
        const getPair = (i: number) => {
          const result: BattleStatePlayer[] = [];
          _pairs[i].tmgIds.forEach((player) => {
            if (battle.players[player]) result.push(battle.players[player]);
          });
          return result;
        };

        final.push({ players: getPair(i), pair: pair, id: nanoid(6), idx: i });
      });

      setPairs(final);
    }
  }, [battle]);

  return (
    <ScrollArea className="max-h-80 pr-3 -mr-3">
      <ul className="leading-4 space-y-1.5">
        {pairs.map((pair, i) => (
          <li key={pair.id}>
            <BattleTablePairsRow data={pair} isActive={i === currentPairIdx} />
          </li>
        ))}
      </ul>
    </ScrollArea>
  );
};

const BattleTablePairsRow = ({ data: { pair, players, idx }, isActive }: { data: PairData; isActive: boolean }) => {
  const { setCurrentPairIdx } = useBattle();
  return (
    <button
      className={clsx(
        'flex items-center gap-2 w-full py-1 pr-2 pl-4 bg-gradient-to-b to-transparent transition-colors rounded-[30px] overflow-hidden',
        isActive ? 'from-primary hover:bg-primary/15' : 'from-white/15 hover:bg-white/15',
      )}
      onClick={() => setCurrentPairIdx(idx)}>
      <span
        className={clsx(
          'w-2 h-2 rounded-full',
          pair.gameIsOver ? 'bg-error' : 'bg-primary shadow-[0_0_10px] shadow-primary',
        )}
      />
      <span className="flex items-center gap-3 text-[12px] leading-[18px]">
        <span
          className={clsx(
            'w-20 truncate text-right',
            pair.gameIsOver ? (pair.winner === players[0].tmgId ? 'text-primary' : 'text-error') : '',
          )}>
          {players[0].name}
        </span>
        <Icon name="swords" className="w-3.5 h-3.5" />
        <span
          className={clsx(
            'w-20 truncate text-left',
            pair.gameIsOver ? (pair.winner === players[1].tmgId ? 'text-primary' : 'text-error') : '',
          )}>
          {players[1].name}
        </span>
      </span>
      <span className="inline-flex justify-center items-center ml-auto min-w-[90px] text-center uppercase tracking-[.04em] font-kanit">
        {pair.gameIsOver ? (
          <span className="btn py-1.5 px-4 text-[12px] leading-none italic">Finished</span>
        ) : isActive ? (
          <span className="btn py-1.5 px-4 text-[12px] leading-none italic">Live</span>
        ) : (
          <span className="btn py-1.5 px-4 text-[12px] leading-none bg-primary">ongoing</span>
        )}
      </span>
    </button>
  );
};
