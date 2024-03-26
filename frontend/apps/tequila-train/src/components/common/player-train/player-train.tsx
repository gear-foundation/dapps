import { Popover } from '@headlessui/react';
import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { getBgColors } from 'app/utils';

type Props = {
  index: number;
};
export const PlayerTrain = ({ index }: Props) => {
  return (
    <Popover className="relative">
      <Popover.Button>
        <Icon name="train" width={43} height={35} className={clsx('w-full h-auto', getBgColors(index).train)} />
      </Popover.Button>

      <Popover.Panel className="absolute z-10 w-max max-w-sm">
        <div className="grid gap-3 p-7 overflow-hidden rounded-lg shadow-lg bg-[#D6FE51] border-2 border-[#1E942A]">
          <h3 className="font-kanit font-bold text-lg">Are you sure?</h3>
          <p className="text-dark-400 font-medium">
            You have to drink a shot if you want to get your train out of the track.
          </p>

          <div className="grid grid-cols-2 items-center gap-2 mt-2">
            <Popover.Button className="btn btn--primary py-1.5 grow">
              Drink!
            </Popover.Button>
            <Popover.Button className="btn btn--black py-1.5 grow">No, thanks</Popover.Button>
          </div>
        </div>
      </Popover.Panel>
    </Popover>
  );
};
