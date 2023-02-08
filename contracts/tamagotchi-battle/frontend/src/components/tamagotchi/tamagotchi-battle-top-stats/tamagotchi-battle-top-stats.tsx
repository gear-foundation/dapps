import clsx from 'clsx';
import { TamagotchiAvatar } from '../tamagotchi-avatar';
import { BattlePlayerType, BattleStatesList } from 'app/types/battles';
import { Icon } from '../../ui/icon';
import { useEffect, useState } from 'react';

type Props = {
  tamagotchi: BattlePlayerType;
  isWinner?: boolean;
  state: BattleStatesList;
  isReverse?: boolean;
  children?: JSX.Element;
};
export const TamagotchiBattleTopStats = ({ isWinner, state, tamagotchi, isReverse, children }: Props) => {
  const [dead, setDead] = useState(false);

  useEffect(() => {
    if (!tamagotchi.health) {
      setDead(!tamagotchi.health);
    }
  }, [tamagotchi]);

  return (
    <div className={clsx('basis-[445px] flex gap-6 items-center', isReverse && 'flex-row-reverse')}>
      <div className="relative flex flex-col items-center w-fit">
        <div
          className={clsx(
            'relative w-15 xl:w-24 aspect-square rounded-full overflow-hidden ring-2 ring-opacity-50',
            dead ? 'bg-error ring-error' : 'bg-white ring-white',
          )}>
          <TamagotchiAvatar
            inBattle
            className="w-30 xl:w-48 aspect-square -left-1/2"
            age={'baby'}
            hasItem={[]}
            color={tamagotchi.color}
            isDead={dead}
          />
        </div>
      </div>
      <div className="w-full max-w-[300px] space-y-3">
        <div className={clsx('relative py-0.5 px-4 rounded-xl overflow-hidden', dead ? 'bg-error' : 'bg-white/10')}>
          {!dead && (
            <div className="absolute inset-0 rounded-xl bg-primary" style={{ width: `${tamagotchi.health / 25}%` }} />
          )}
          <div className="relative flex gap-1 items-center justify-center">
            <Icon name="health" className="w-3.5 h-3.5" />
            <span className="font-kanit text-xs font-medium leading-5">{Math.round(tamagotchi.health / 250)} / 10</span>
          </div>
        </div>
        <div className={clsx('flex gap-3', isReverse && 'flex-row-reverse')}>
          <div className="relative flex gap-1 items-center font-kanit text-xs font-medium leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="armor" className="w-3.5 h-3.5" />
            {/*<h4>Defence: </h4>*/}
            <p className="flex gap-1 items-center">
              <span>{Math.round(tamagotchi.defence / 1000)}</span>
            </p>
          </div>
          <div className="relative flex gap-1 items-center font-kanit text-xs font-medium leading-5 bg-white/10 py-0.5 px-4 rounded-xl">
            <Icon name="wins" className="w-3.5 h-3.5" />
            {/*<h4>Strength: </h4>*/}
            <p className="flex gap-1 items-center">
              <span>{Math.round(tamagotchi.power / 1000)}</span>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
