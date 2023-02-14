import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { getBgColors } from 'app/utils';
import { DominoItem } from '../../common/domino-item';
import { DominoZone } from '../../common/domino-zone';
import { DominoTileType } from 'app/types/game';

const players = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];

type Props = {
  index: number;
  train?: boolean;
  isUserTrain?: boolean;
  active?: boolean;
  tiles?: DominoTileType[];
};
export const PlayerTrackSection = ({ index, train, isUserTrain, active, tiles }: Props) => {
  return (
    <div
      className={clsx(
        'relative grid grid-cols-[170px_1fr] grid-rows-[36px] gap-10 py-3 px-3.5 rounded-lg overflow-hidden',
        'before:absolute before:inset-0',
        active
          ? 'before:bg-gradient-to-r before:from-[white_-3%] before:to-[transparent_24.7%] before:opacity-50'
          : 'before:bg-[#EBF1EE] before:bg-opacity-90',
        getBgColors(index).backdrop,
      )}>
      <div className="relative grid grid-cols-[44px_1fr] items-center gap-3">
        {(isUserTrain || train) && (
          <Icon
            name="train"
            width={43}
            height={35}
            className={clsx('w-full h-auto', train ? 'text-[#FFCE4A]' : getBgColors(index).train)}
          />
        )}
        <span
          className={clsx(
            'uppercase leading-4 font-semibold tracking-[0.03em] w-min',
            active && !getBgColors(index).isLight && 'text-white',
          )}>
          {train ? 'Tequila Train' : `Señor ${players[index]}`}
        </span>
      </div>
      <div className="relative flex items-center gap-0.5">
        {tiles && tiles.map((tile, i) => <DominoItem row tile={tile} key={i} />)}

        {(active || train) && <DominoZone light={active && !getBgColors(index).isLight} />}
      </div>
    </div>
  );
};
