import { DominoItem } from '../../common/domino-item';

export const PlayerConsSection = () => {
  return (
    <div className="flex justify-between bg-[#D6FE51] py-3 px-7 rounded-2xl border border-dark-500 border-opacity-15">
      <div className="flex flex-wrap items-center gap-2">
        <DominoItem />
        <DominoItem />
        <DominoItem />
        <DominoItem />
        <DominoItem />
        <DominoItem />
        <DominoItem />
      </div>
      <div className="py-1 border-l border-primary pl-6 flex flex-col gap-3 min-w-[175px]">
        <button className="btn btn--primary text-dark-500">Draw</button>
        <button className="btn btn--black">Pass</button>
      </div>
    </div>
  );
};
