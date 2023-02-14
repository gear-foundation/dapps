import { Icon } from '../../ui/icon';
import clsx from 'clsx';

type Props = {
  row?: boolean;
};
export const DominoItem = ({ row }: Props) => {
  return (
    <div className={clsx(row && 'flex')}>
      <div
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? ' rounded-l-lg' : ' rounded-t-lg',
        )}>
        <Icon name="domino-1" section="domino" width={27} height={27} className={clsx(row && 'rotate-90')} />
      </div>
      <div
        className={clsx(
          'flex items-center justify-center w-9 h-9 bg-white border border-[#1E942A]',
          row ? 'rounded-r-lg' : 'rounded-b-lg',
        )}>
        <Icon name="domino-2" section="domino" width={27} height={27} className={clsx(row && 'rotate-90')} />
      </div>
    </div>
  );
};
