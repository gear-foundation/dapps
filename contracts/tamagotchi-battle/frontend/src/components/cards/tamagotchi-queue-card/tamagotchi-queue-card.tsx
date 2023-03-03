import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';
import clsx from 'clsx';
import { Icon } from 'components/ui/icon';
import type { BattleStatePlayer } from 'app/types/battles';

type TamagotchiQueueCardProps = {
  className?: string;
  tamagotchi: BattleStatePlayer;
  asPlayer?: boolean;
  isActive?: boolean;
};

export const TamagotchiQueueCard = ({ className, tamagotchi, asPlayer, isActive }: TamagotchiQueueCardProps) => {
  const dead = !tamagotchi.health;

  return (
    <div
      className={clsx(
        'relative grid justify-center pt-3 pb-4 xxl:py-4 px-4 xxl:px-5',
        asPlayer ? 'w-33 xxl:w-40 smh:gap-1 gap-1.5 xxl:gap-4' : 'gap-1 xxl:gap-2 w-full bg-[#29292B] rounded-2xl',
        className,
        dead && !asPlayer && 'opacity-30',
      )}>
      {asPlayer && (
        <div
          className={clsx(
            'absolute inset-x-0 -top-4 xxl:-top-8 -bottom-2 -z-1 w-full card-mask overflow-visible',
            'bg-gradient-to-b to-transparent',
            isActive ? 'from-[#16B768]' : 'from-theme-blue',
          )}
        />
      )}

      {dead && (
        <Icon name="message-rip" className="absolute top-6 right-3 xxl:top-10 xxl:right-2 w-5 xxl:w-6 aspect-square" />
      )}
      <div className="relative w-14 xxl:w-24 aspect-square m-auto rounded-full overflow-hidden bg-white ring-white ring-4 ring-opacity-10">
        <TamagotchiAvatar
          className="w-28 xxl:w-48 aspect-square -left-1/2"
          age={tamagotchi.dateOfBirth}
          color={tamagotchi.color}
          isDead={dead}
        />
      </div>
      <h3
        className={clsx(
          'flex justify-center text-center tracking-[0.03em] font-medium',
          asPlayer ? 'text-lg leading-7' : 'text-sm',
        )}>
        <span className="block truncate max-w-[10ch]">{tamagotchi.name ? tamagotchi.name : 'Geary'}</span>
      </h3>
      <div
        className={clsx('relative w-full xxl:w-30 px-4 rounded-xl overflow-hidden', dead ? 'bg-error' : 'bg-white/10')}>
        {!dead && (
          <div className="absolute inset-0 rounded-xl bg-primary" style={{ width: `${tamagotchi.health / 25}%` }} />
        )}
        <div className="relative flex gap-1 items-center justify-center">
          <Icon name="health" className="w-3 xxl:w-3.5 aspect-square" />
          <span className="font-kanit text-xs font-medium leading-5">{Math.round(tamagotchi.health / 25)} / 100</span>
        </div>
      </div>
    </div>
  );
};
