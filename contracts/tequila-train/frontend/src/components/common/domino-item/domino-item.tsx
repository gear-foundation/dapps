import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { DominoTileType } from 'app/types/game';

type Props = {
  row?: boolean;
  tile: DominoTileType;
};
export const DominoItem = ({ row, tile }: Props) => {
  return (
    <div className={clsx(row && 'flex')}>
      <div
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? ' rounded-l-lg' : ' rounded-t-lg',
        )}>
        {tile && tile[0] && (
          <Icon
            name={`domino-${tile[0]}`}
            section="domino"
            width={27}
            height={27}
            className={clsx(row && 'rotate-90')}
          />
        )}
      </div>
      <div
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? 'rounded-r-lg' : 'rounded-b-lg',
        )}>
        {tile && tile[1] && (
          <Icon
            name={`domino-${tile[1]}`}
            section="domino"
            width={27}
            height={27}
            className={clsx(row && 'rotate-90')}
          />
        )}
      </div>
    </div>
  );
};
