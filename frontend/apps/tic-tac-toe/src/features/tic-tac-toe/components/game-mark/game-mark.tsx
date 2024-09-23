import { Mark } from '@/app/utils';
import { PlayerIconCircle, PlayerIconCross } from '../../assets';
import { BaseComponentProps } from '@/app/types';

type PlayerMarkProps = BaseComponentProps & {
  mark: Mark;
};

export function GameMark({ mark, className }: PlayerMarkProps) {
  return mark === 'X' ? <PlayerIconCross className={className} /> : <PlayerIconCircle className={className} />;
}
