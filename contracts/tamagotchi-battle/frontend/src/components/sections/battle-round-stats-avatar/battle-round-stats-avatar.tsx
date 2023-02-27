import clsx from 'clsx';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';
import { BattleStatePlayer } from 'app/types/battles';
import { Icon } from 'components/ui/icon';
import { useEffect, useState } from 'react';
import { motion } from 'framer-motion';

type Props = {
  tamagotchi: BattleStatePlayer;
  isReverse?: boolean;
};
export const BattleRoundStatsAvatar = ({ tamagotchi, isReverse }: Props) => {
  const [dead, setDead] = useState(false);

  useEffect(() => {
    setDead(!tamagotchi.health);
    return () => setDead(false);
  }, [tamagotchi]);

  return (
    <motion.div
      initial={{ opacity: 0, x: isReverse ? 100 : -100 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: isReverse ? 100 : -100 }}
      transition={{ duration: 0.5 }}
      className={clsx('basis-[40%] flex gap-6 items-center', isReverse && 'flex-row-reverse')}>
      <div className="relative flex flex-col items-center w-fit">
        <div
          className={clsx(
            'relative w-15 xxl:w-24 aspect-square rounded-full overflow-hidden ring-2 ring-opacity-50',
            dead ? 'bg-error ring-error' : 'bg-white ring-white',
          )}>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.5 }}>
            <TamagotchiAvatar
              className="w-30 xxl:w-48 aspect-square -left-1/2"
              age={tamagotchi.dateOfBirth}
              color={tamagotchi.color}
              isDead={dead}
            />
          </motion.div>
        </div>

        <div className="absolute top-[calc(100%-8px)] inline-flex gap-2 items-center py-0.5 px-2.5 xxl:py-1 xxl:px-5 tracking-widest font-kanit font-semibold xxl:text-base leading-4 xxl:leading-5 bg-[#404040] rounded-lg">
          <Icon name="wins" className="w-3.5 h-3.5 xxl:w-5 xxl:h-5" /> {tamagotchi.victories}
        </div>
      </div>
      <div className="w-full max-w-[300px] space-y-3">
        <div className={clsx('relative py-0.5 px-4 rounded-xl overflow-hidden', dead ? 'bg-error' : 'bg-white/10')}>
          {!dead && (
            <div
              className={clsx(
                'absolute inset-y-0 w-full rounded-xl bg-primary transition-[width]',
                isReverse ? 'right-0' : 'left-0',
              )}
              style={{ width: `${tamagotchi.health / 25}%` }}
            />
          )}
          <div className="relative flex gap-2 items-center justify-center">
            <Icon name="health" className="w-3 xxl:w-3.5 aspect-square" />
            <span className="font-kanit text-xs font-medium leading-5">{Math.round(tamagotchi.health / 25)} / 100</span>
          </div>
        </div>
        <div className={clsx('flex gap-3 tracking-[0.03em]', isReverse && 'flex-row-reverse')}>
          <div className="relative z-1 flex gap-1.5 items-center font-medium font-kanit text-xs leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="armor" className="w-3 xxl:w-3.5 aspect-square" />
            <b className="font-bold">{Math.round(tamagotchi.defence / 100)}</b> Armor
          </div>
          <div className="relative z-1 flex gap-1.5 items-center font-medium font-kanit text-xs leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="strength" className="w-3 xxl:w-3.5 aspect-square" />
            <b className="font-bold">{Math.round(tamagotchi.power / 100)}</b> Strength
          </div>
        </div>
      </div>
    </motion.div>
  );
};
