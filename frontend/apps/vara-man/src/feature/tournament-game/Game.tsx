import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';
import { useAccount } from '@gear-js/react-hooks';

import { useGame } from '@/app/context/ctx-game';

import { Icons } from '@/components/ui/icons';
import { GameTimer } from './components/timer';
import { GameLayout } from './GameLayout';
import { Registration } from './components/registration';
import { GamePlayers } from './components/game-players';
import { GameOverModal } from './components/modals/game-over';
import { GameCanceledModal } from './components/modals/game-canceled';

import { calculatePoints } from '../game/utils/calculatePoints';
import { COINS, GAME_OVER } from '../game/consts';

import { IGameLevel } from '@/app/types/game';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useMediaQuery } from '@/hooks/use-mobile-device';

export const Game = () => {
  const { account } = useAccount();
  const [isCanceledModal, setCanceledModal] = useState(false);
  const [activeTab, setActiveTab] = useState('leaderboard');
  const [playGame, setPlayGame] = useState(false);

  const { tournamentGame, previousGame, setPreviousGame } = useGame();

  const [gameOver, setGameOver] = useAtom(GAME_OVER);
  const { configState } = useGame();
  const [coins] = useAtom(COINS);

  const level = tournamentGame?.[0].level || previousGame?.[0].level;
  const score = configState && calculatePoints(coins, configState, level as IGameLevel);

  const isRegistration = tournamentGame?.[0].stage === 'Registration' || previousGame?.[0].stage === 'Registration';
  const isFinished = tournamentGame?.[0].stage.Finished || previousGame?.[0].stage.Finished;
  const isStarted = tournamentGame?.[0].stage.Started || previousGame?.[0].stage.Started;

  useEffect(() => {
    const admin = tournamentGame?.[0].admin || previousGame?.[0].admin;
    const isAdmin = admin === account?.decodedAddress;

    if (previousGame && !tournamentGame) {
      if (!isAdmin) {
        setCanceledModal(true);
      } else {
        setPreviousGame(null);
      }
    }
  }, [tournamentGame]);

  useEffect(() => {
    if (playGame || isStarted) {
      setActiveTab('play');
    }
  }, [playGame, isStarted]);

  return (
    <div className="grid gap-4 grid-cols-1 md:grid-cols-3">
      <div className="tabs-mobile md:hidden">
        <Tabs className="flex flex-col" value={activeTab}>
          <TabsList className="flex border-b">
            <TabsTrigger
              className="bg-white px-5 h-[45px] font-semibold flex-1 flex items-center justify-center data-[state=active]:border-b data-[state=active]:border-b-[#00FFC4]"
              value="play"
              onClick={() => setActiveTab('play')}>
              Play
            </TabsTrigger>
            <TabsTrigger
              className="bg-white px-5 h-[45px] font-semibold flex-1 flex items-center justify-center data-[state=active]:border-b data-[state=active]:border-b-[#00FFC4]"
              value="leaderboard"
              onClick={() => setActiveTab('leaderboard')}>
              Leaderboard
            </TabsTrigger>
          </TabsList>
          <div>
            <TabsContent value="leaderboard" className="grow py-5 bg-white rounded-b-md outline-none">
              {isRegistration && previousGame && (
                <Registration setPlayGame={setPlayGame} tournamentGame={previousGame} />
              )}
              {isStarted && <GamePlayers />}
            </TabsContent>
            <TabsContent value="play" className="grow bg-white rounded-b-md outline-none">
              <GameInfoCanvas
                isStarted={isStarted}
                isRegistration={isRegistration}
                isFinished={isFinished}
                gameOver={gameOver}
                setGameOver={setGameOver}
                score={score}
              />
            </TabsContent>
          </div>
        </Tabs>
      </div>

      <div className="hidden md:flex md:col-span-1 lg:col-span-1 p-5 bg-white rounded-md">
        {isRegistration && previousGame && <Registration tournamentGame={previousGame} />}
        {isStarted && <GamePlayers />}
      </div>

      <div className="hidden md:flex md:col-span-2 lg:col-span-2 p-5 bg-white rounded-md">
        <GameInfoCanvas
          isStarted={isStarted}
          isRegistration={isRegistration}
          isFinished={isFinished}
          gameOver={gameOver}
          setGameOver={setGameOver}
          score={score}
        />
      </div>

      {isFinished && tournamentGame && <GameOverModal tournamentGame={tournamentGame} />}
      {isCanceledModal && <GameCanceledModal />}
    </div>
  );
};

type GameInfoCanvasProps = {
  isStarted: boolean;
  isRegistration: boolean;
  isFinished: boolean;
  gameOver: boolean;
  setGameOver: (value: boolean) => void;
  score: number | undefined | null;
};

const GameInfoCanvas = ({
  isStarted,
  isRegistration,
  isFinished,
  gameOver,
  setGameOver,
  score,
}: GameInfoCanvasProps) => {
  const isMobile = useMediaQuery('(max-width: 768px)');

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
          <div className="flex gap-3 items-center font-semibold cursor-pointer" onClick={() => setGameOver(true)}>
            <Icons.exit />
            Exit
          </div>
        </div>
      )}
      <GameLayout isPause={isRegistration || isFinished || !isStarted} />
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
