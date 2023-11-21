import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { getBgColors } from 'app/utils';
import { useGame } from '../../../app/context';

type Props = {
  index: number;
  active: boolean;
};

export const PlayerCardSection = ({ index, active }: Props) => {
  const { gameWasm: wasm } = useGame();
  return (
    <div className="relative flex flex-col h-full max-w-[160px]">
      {active && (
        <div
          className={clsx(
            'absolute -z-1 bottom-0 -inset-x-px h-[calc(100%+1rem)] -mt-1 bg-[#D6FE51] border-x border-dark-500/15',
            'before:absolute before:top-0 before:right-full before:w-10 before:h-10 before:bg-[#ebf1ee] before:rounded-tr-3xl before:border-r before:border-t before:border-dark-500/15',
            'after:absolute after:top-0 after:left-full after:w-10 after:h-10 after:bg-[#ebf1ee] after:rounded-tl-3xl after:border-l after:border-t after:border-dark-500/15',
          )}>
          <div
            className={clsx(
              'before:absolute before:top-0 before:right-full before:-z-1 before:w-10 before:h-10 before:bg-[#D6FE51]',
              'after:absolute after:top-0 after:left-full after:-z-1 after:w-10 after:h-10 after:bg-[#D6FE51]',
            )}
          />
        </div>
      )}
      <div className="grow flex rounded-t-2xl bg-[#D6FE51] py-3.5 px-2.5 font-medium text-center">
        <span className="line-clamp-2 w-full">{wasm?.players[index][1]}</span>
      </div>
      <div
        className={clsx(
          'grow-0 flex items-center py-3.5 px-2.5',
          getBgColors(index).backdrop,
          getBgColors(index).isLight ? 'text-dark-500' : 'text-white',
        )}>
        <div className="flex flex-wrap gap-1 items-center justify-center">
          <div
            className={clsx(
              'inline-flex items-center justify-center w-6 h-6 rounded-full',
              getBgColors(index).isLight ? 'bg-black/10' : 'bg-white/15',
            )}>
            <Icon name="on-hands" className="m-auto w-3 h-3" />
          </div>
          <span className="font-bold text-lg">{wasm?.playersTiles[index].length}</span>
          <span className="basis-full text-center whitespace-nowrap text-[8px] leading-3">On hands</span>
        </div>
        <div className="flex flex-wrap gap-1 items-center justify-center">
          <div
            className={clsx(
              'inline-flex items-center justify-center w-6 h-6 rounded-full',
              getBgColors(index).isLight ? 'bg-black/10' : 'bg-white/15',
            )}>
            <Icon name="shots" className="m-auto w-4 h-4" />
          </div>
          <span className="font-bold text-lg">{wasm?.shotCounters[index]}</span>
          <span className="basis-full text-center whitespace-nowrap text-[8px] leading-3">Number of shots</span>
        </div>
      </div>
    </div>
  );
};
