import clsx from 'clsx';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';
import { BattlePlayerType } from 'app/types/battles';
import { Icon } from 'components/ui/icon';
import { useEffect, useState } from 'react';

type Props = {
  tamagotchi: BattlePlayerType;
  isReverse?: boolean;
};
export const BattleRoundStatsAvatar = ({ tamagotchi, isReverse }: Props) => {
  const [dead, setDead] = useState(false);

  useEffect(() => {
    setDead(!tamagotchi.health);
    return () => setDead(false);
  }, [tamagotchi]);

  return (
    <div className={clsx('basis-[40%] flex gap-6 items-center', isReverse && 'flex-row-reverse')}>
      <div className="relative flex flex-col items-center w-fit">
        <div
          className={clsx(
            'relative w-15 xxl:w-24 aspect-square rounded-full overflow-hidden ring-2 ring-opacity-50',
            dead ? 'bg-error ring-error' : 'bg-white ring-white',
          )}>
          <TamagotchiAvatar
            className="w-30 xxl:w-48 aspect-square -left-1/2"
            age={tamagotchi.dateOfBirth}
            color={tamagotchi.color}
            isDead={dead}
          />
        </div>
      </div>
      <div className="w-full max-w-[300px] space-y-3">
        <div className={clsx('relative py-0.5 px-4 rounded-xl overflow-hidden', dead ? 'bg-error' : 'bg-white/10')}>
          {!dead && (
            <div
              className="absolute inset-0 rounded-xl bg-primary transition-[width]"
              style={{ width: `${tamagotchi.health / 25}%` }}
            />
          )}
          <div className="relative flex gap-1 items-center justify-center">
            <Icon name="health" className="w-3.5 h-3.5" />
            <span className="font-kanit text-xs font-medium leading-5">{Math.round(tamagotchi.health / 25)} / 100</span>
          </div>
        </div>
        <div className={clsx('flex gap-3 tracking-[0.03em]', isReverse && 'flex-row-reverse')}>
          <div className="relative flex gap-1.5 items-center font-medium font-kanit text-xs leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="armor" className="w-3.5 h-3.5" />
            <b className="font-bold">{Math.round(tamagotchi.defence / 100)}</b> Armor
          </div>
          <div className="relative flex gap-1.5 items-center font-medium font-kanit text-xs leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="wins" className="w-3.5 h-3.5" />
            <b className="font-bold">{Math.round(tamagotchi.power / 100)}</b> Strength
          </div>
        </div>
      </div>
    </div>
  );
};
