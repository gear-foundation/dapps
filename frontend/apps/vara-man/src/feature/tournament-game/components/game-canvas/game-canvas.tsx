import { Icons } from '@/components/ui/icons';
import { useMediaQuery } from '@/hooks/use-mobile-device';
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';
import { GameLayout } from '../../GameLayout';
import { GameTimer } from '../timer';
import { MOBILE_BREAKPOINT } from '@/app/consts';

type GameInfoCanvasProps = {
  isStarted: boolean;
  isRegistration: boolean;
  isFinished: boolean;
  gameOver: boolean;
  isCanceledModal: boolean;
  score: number | undefined | null;
};

export const GameInfoCanvas = ({
  isStarted,
  isRegistration,
  isFinished,
  gameOver,
  isCanceledModal,
  score,
}: GameInfoCanvasProps) => {
  const isMobile = useMediaQuery(MOBILE_BREAKPOINT);

  return (
    <div className="md:w-full md:flex md:flex-col md:justify-center md:items-center select-none">
      {isStarted && (
        <div className="w-full md:w-[588px] flex justify-between my-3">
          <div className="flex gap-3 items-center">
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsTimer />
              <GameTimer isPause={isRegistration || isFinished || gameOver} />
            </div>
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsCoins />
              {score}
            </div>
          </div>
        </div>
      )}
      <GameLayout isPause={isRegistration || isFinished || !isStarted} isCanceledModal={isCanceledModal} />
      {!isMobile && (
        <div className="flex gap-5 my-3">
          <div className="flex gap-3 items-center">
            <div className="bg-[#DFDFDF] rounded-sm p-1">
              <ArrowUp color="#767676" />
            </div>
            <div className="bg-[#DFDFDF] rounded-sm p-1">
              <ArrowDown color="#767676" />
            </div>
            <span>Use arrows to move</span>
          </div>
          <div className="flex gap-3 items-center">
            <div className="bg-[#DFDFDF] rounded-sm p-1">
              <ArrowLeft color="#767676" />
            </div>
            <div className="bg-[#DFDFDF] rounded-sm p-1">
              <ArrowRight color="#767676" />
            </div>
            <span>Rotate</span>
          </div>
          <div className="flex gap-3 items-center">
            <div className="bg-[#DFDFDF] rounded-sm p-1 px-3 font-bold text-[#726F6F]">Shift</div>
            <span>Hold shift to run</span>
          </div>
        </div>
      )}
    </div>
  );
};
