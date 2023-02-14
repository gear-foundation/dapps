import { Icon } from '../../ui/icon';
import clsx from 'clsx';
import { DominoNumber } from 'app/types/game';

type Props = {
  row?: boolean;
  left?: DominoNumber;
  right?: DominoNumber;
};
export const DominoItem = ({ row, left, right }: Props) => {
  return (
    <div className={clsx(row && 'flex')}>
      <div
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? ' rounded-l-lg' : ' rounded-t-lg',
        )}>
        {left && left !== 'Zero' && (
          <Icon
            name={`domino-${left.toLowerCase()}`}
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
        {right && right !== 'Zero' && (
          <Icon
            name={`domino-${right.toLowerCase()}`}
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
