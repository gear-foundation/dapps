import { useSearchParams } from 'react-router-dom';
import { useAtom } from 'jotai';
import { useState } from 'react';
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';
import { GameCanvas } from './GameCanvas';
import { useGame } from '@/app/context/ctx-game';
import { Icons } from '@/components/ui/icons';
import { GameTimer } from './components/timer';
import { IGameLevel } from '@/app/types/game';
import { calculatePoints } from '../game/utils/calculatePoints';
import { COINS, GAME_OVER } from '../game/consts';

export const Game = () => {
  const [searchParams] = useSearchParams();
  const [, setGameOver] = useAtom(GAME_OVER);
  const { configState } = useGame();
  const [coins] = useAtom(COINS);
  const [gameRestarted, setGameRestarted] = useState(false);

  const score = configState && calculatePoints(coins, configState, searchParams.get('level') as IGameLevel);

  const handleGameRestart = () => {
    setGameRestarted((prev) => !prev); 
    setGameOver(false); 
  };

  return (
    <div>
      <div className="w-full flex flex-col justify-center items-center">
        <div className="w-[588px] flex justify-between my-3">
          <div className="flex gap-3 items-center">
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsTimer />
              <GameTimer gameRestarted={gameRestarted} />
            </div>
            <div className="flex gap-3 items-center font-semibold">
              <Icons.statsCoins />
              {score}
            </div>
          </div>
          <div className="flex gap-3 items-center font-semibold cursor-pointer" onClick={() => setGameOver(true)}>
            <Icons.exit />
            Exit
          </div>
        </div>
        <GameCanvas onRestart={handleGameRestart} />
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
      </div>
    </div>
  );
};
