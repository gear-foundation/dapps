import { TamagotchiAvatar } from '../../tamagotchi/tamagotchi-avatar';
import clsx from 'clsx';
import { Icon } from '../../ui/icon';

type TamagotchiQueueCardProps = {
  className?: string;
  isDead?: boolean;
};

export const TamagotchiQueueCard = ({ className, isDead }: TamagotchiQueueCardProps) => {
  return (
    <div
      className={clsx('relative grid gap-2 justify-center w-fit py-4 px-5 bg-[#29292B] w-fit rounded-2xl', className)}>
      <div className="relative w-15 xl:w-24 aspect-square m-auto rounded-full overflow-hidden ring-4 ring-opacity-10 bg-white ring-white">
        <TamagotchiAvatar
          inBattle
          className="w-30 xl:w-48 aspect-square -left-1/2"
          age={'baby'}
          hasItem={[]}
          isDead={isDead}
        />
      </div>
      <h3 className="flex justify-center text-center tracking-[0.03em] text-sm font-medium">
        <span className="block truncate max-w-[10ch]">Geary</span>
      </h3>
      <div className="w-full max-w-[300px] space-y-3">
        <div className="relative w-30 px-4 rounded-xl overflow-hidden bg-white/10">
          <div
            className={clsx('absolute inset-0 rounded-xl', !isDead ? 'bg-primary' : 'bg-error')}
            style={{ width: `${50}%` }}
          />
          <div className="relative flex gap-1 items-center justify-center">
            <Icon name="health" className="w-3.5 h-3.5" />
            <span className="font-kanit text-xs font-medium leading-5">{50} / 100</span>
          </div>
        </div>
      </div>
    </div>
  );
};
