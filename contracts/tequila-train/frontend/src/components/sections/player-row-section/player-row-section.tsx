import { Icon } from '../../ui/icon';
import clsx from 'clsx';

const players = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
const colors = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];

const getBgColors = (v: number) => {
  switch (v) {
    case 0:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#EB5757]' };
    case 1:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#0075A7]' };
    case 2:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#F2C34C]' };
    case 3:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#F0FE51]' };
    case 4:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#353535]' };
    case 5:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#219653]' };
    case 6:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#00D1FF]' };
    case 7:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#670E9D]' };
    case 8:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#67CB4D]' };
    default:
      return { row: 'bg-[#EBF1EE]', backdrop: 'bg-[#67CB4D]' };
  }
};

type Props = {
  index: number;
  train?: boolean;
  isUserTrain?: boolean;
};
export const PlayerRowSection = ({ index, train, isUserTrain }: Props) => {
  return (
    <div
      className={clsx(
        'relative grid grid-cols-[170px_1fr] gap-10 py-3 px-3.5 rounded-lg overflow-hidden bg-opacity-90',
        getBgColors(index).row,
      )}>
      <div className={clsx('absolute inset-0 -z-1', getBgColors(index).backdrop)} />
      <div className="grid grid-cols-[44px_1fr] items-center gap-3">
        {(isUserTrain || train) && (
          <Icon name="train" width={43} height={35} className="w-full h-auto text-[#FFCE4A]" />
        )}
        <span className="uppercase leading-4 font-semibold tracking-[0.03em] w-min">
          {train ? 'Tequila Train' : `Señor ${players[index]}`}
        </span>
      </div>
      <div className="">fields</div>
    </div>
  );
};
