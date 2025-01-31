import { DominoItem } from '../domino-item';
import { DominoTileType } from '@/app/types/game';
import clsx from 'clsx';

type Props = {
  tile: DominoTileType;
  onClick: () => void;
  isSelected?: boolean;
};

export const PlayerDomino = ({ tile, onClick, isSelected }: Props) => {
  return (
    <button className={clsx('transition-transform', isSelected && '-translate-y-7')} onClick={onClick}>
      <DominoItem tile={tile} />
    </button>
  );
};
