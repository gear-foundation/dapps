import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAtom } from 'jotai';
import { useEffect } from 'react';
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';
import { Game } from './Game';
import { useGame } from '@/app/context/ctx-game';
import { Icons } from '@/components/ui/icons';
import { GameTimer } from './components/timer';
import { IGameLevel } from '@/app/types/game';
import { calculatePoints } from '../game/utils/calculatePoints';
import { COINS, GAME_OVER, gameLevels } from '../game/consts';
import { useMediaQuery } from '@/hooks/use-mobile-device';
import { MOBILE_BREAKPOINT } from '@/app/consts';

export const GameLayout = () => {
  const isMobile = useMediaQuery(MOBILE_BREAKPOINT);
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const [, setGameOver] = useAtom(GAME_OVER);
  const { configState } = useGame();
  const [coins] = useAtom(COINS);
  const level = searchParams.get('level') as IGameLevel;
  const currentLevel = level || gameLevels.find((l) => l.level === level) !== undefined;

  const score = configState && calculatePoints(coins, configState, level);

  useEffect(() => {
    if (!currentLevel) {
      navigate('/');
    }
  }, [level]);

  return (
    <div>
      <div className="md:w-full md:flex md:flex-col md:justify-center md:items-center select-none">
        <div className="w-full md:w-[588px] flex justify-between my-3">
          <div className="flex gap-3 items-center">
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsTimer />
              <GameTimer />
            </div>
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsCoins />
              {score}
            </div>
          </div>
          <div className="flex gap-3 items-center font-semibold cursor-pointer" onClick={() => navigate('/')}>
            <Icons.exit />
            Exit
          </div>
        </div>
        <Game />
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
    </div>
  );
};
