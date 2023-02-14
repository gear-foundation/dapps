import clsx from 'clsx';

type Props = {
  light?: boolean;
};

export const DominoZone = ({ light }: Props) => {
  return (
    <div
      className={clsx(
        'w-19 h-9 border  border-dashed rounded-lg',
        light ? 'bg-white/15 border-white' : 'bg-black/15 border-black',
      )}
    />
  );
};
