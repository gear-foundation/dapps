import { BattleStatePlayer } from 'app/types/battles';
import clsx from 'clsx';
import { Icon } from 'components/ui/icon';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';

type TamagotchiBattleInfoCardProps = {
  tamagotchi: BattleStatePlayer;
  isActive: boolean;
};
export const TamagotchiBattleInfoCard = ({ tamagotchi, isActive }: TamagotchiBattleInfoCardProps) => {
  return (
    <div className="relative grid gap-1.5 xxl:gap-4 justify-center w-35 xxl:w-40 pt-7 xxl:pt-11 pb-6 px-3 xxl:px-5 ">
      <div
        className={clsx(
          'absolute inset-x-0 -top-2 xxl:-top-0 bottom-0 w-full card-mask overflow-visible',
          'bg-gradient-to-b to-transparent',
          isActive ? 'from-[#16B768]' : 'from-theme-blue',
        )}
      />

      {!tamagotchi.health && (
        <Icon name="message-rip" className="absolute top-6 right-3 xxl:top-10 xxl:right-2 w-5 xxl:w-6 h-5 xxl:h-6" />
      )}
      <div
        className={clsx(
          'relative w-15 xxl:w-24 aspect-square m-auto rounded-full overflow-hidden ring-4 ring-opacity-10',
          !tamagotchi.health ? 'bg-error ring-error' : 'bg-white ring-white',
        )}>
        <TamagotchiAvatar
          className="w-30 xxl:w-48 aspect-square -left-1/2"
          age={tamagotchi.dateOfBirth}
          color={tamagotchi.color}
          isDead={!tamagotchi.health}
        />
      </div>
      <h3 className="relative flex justify-center text-center tracking-[0.03em] text-lg font-medium leading-7">
        <span className="block truncate max-w-[10ch]">{tamagotchi?.name ? tamagotchi.name : 'Geary'}</span>
      </h3>
      <div
        className={clsx(
          'relative w-full xxl:w-30 px-4 rounded-xl overflow-hidden',
          !tamagotchi.health ? 'bg-error' : 'bg-white/10',
        )}>
        {!!tamagotchi.health && (
          <div className="absolute inset-0 rounded-xl bg-primary" style={{ width: `${tamagotchi.health / 25}%` }} />
        )}
        <div className="relative flex gap-2 items-center justify-center">
          <Icon name="health" className="w-3 xxl:w-3.5 aspect-square" />
          <span className="font-kanit text-xs font-medium leading-5">{Math.round(tamagotchi.health / 25)} / 100</span>
        </div>
      </div>
    </div>
  );
};
