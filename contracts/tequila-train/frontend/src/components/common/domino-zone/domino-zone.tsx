import clsx from 'clsx';
import { useGame } from 'app/context';
import { DominoItem } from '../domino-item';

type Props = {
  light?: boolean;
  disabled?: boolean;
  id: number;
  reverse?: boolean;
};

export const DominoZone = ({ light, id, disabled, reverse }: Props) => {
  const { selectedDomino, setPlayerChoice, playerChoice } = useGame();

  const onClick = () => {
    if (playerChoice) {
      playerChoice.track_id !== id
        ? setPlayerChoice({ ...playerChoice, track_id: id, remove_train: false })
        : setPlayerChoice({
            ...playerChoice,
            track_id: undefined,
            remove_train: false,
          });
    } else {
      setPlayerChoice({ track_id: id, remove_train: false });
    }
  };

  return (
    <button
      className={clsx(
        'inline-flex justify-center items-center w-18 h-9 -m-mx border border-dashed rounded-lg transition-colors',
        'disabled:bg-error/15 disabled:border-error',
        playerChoice?.track_id === id
          ? 'enabled:hover:bg-primary/30 enabled:hover:border-primary'
          : 'enabled:hover:bg-primary/15 enabled:hover:border-primary',
        playerChoice?.track_id === id
          ? 'enabled:bg-primary/15 enabled:border-primary'
          : light
          ? 'enabled:bg-white/15 enabled:border-white'
          : 'enabled:bg-black/15 enabled:border-black',
      )}
      onClick={onClick}
      disabled={disabled}>
      {!disabled && selectedDomino && playerChoice?.track_id === id && (
        <DominoItem row tile={selectedDomino[1]} reverse={reverse} />
      )}
    </button>
  );
};
