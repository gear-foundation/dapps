import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { DominoTileType } from 'app/types/game';

type Props = {
  row?: boolean;
  tile: DominoTileType;
  reverse?: boolean;
};
export const DominoItem = ({ row, tile, reverse }: Props) => {
  return (
    <span className={clsx(row && 'flex')}>
      <span
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? ' rounded-l-lg' : ' rounded-t-lg',
        )}>
        {tile && tile[reverse ? 1 : 0] >= 0 && (
          <Icon
            name={`domino-${tile[reverse ? 1 : 0]}`}
            section="domino"
            width={27}
            height={27}
            className={clsx(row && 'rotate-90')}
          />
        )}
      </span>
      <span
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? 'rounded-r-lg' : 'rounded-b-lg',
        )}>
        {tile && tile[reverse ? 0 : 1] >= 0 && (
          <Icon
            name={`domino-${tile[reverse ? 0 : 1]}`}
            section="domino"
            width={27}
            height={27}
            className={clsx(row && 'rotate-90')}
          />
        )}
      </span>
    </span>
  );
};
