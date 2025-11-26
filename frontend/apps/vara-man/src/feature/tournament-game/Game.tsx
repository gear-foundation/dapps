import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';

import { MOBILE_BREAKPOINT } from '@/app/consts';
import { useGame } from '@/app/context/ctx-game';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useMediaQuery } from '@/hooks/use-mobile-device';

import { COINS, GAME_OVER } from '../game/consts';
import { calculatePoints } from '../game/utils/calculatePoints';

import { GameInfoCanvas } from './components/game-canvas/game-canvas';
import { GamePlayers } from './components/game-players';
import { GameCanceledModal } from './components/modals/game-canceled';
import { GameOverModal } from './components/modals/game-over';
import { Registration } from './components/registration';

export const Game = () => {
  const isMobile = useMediaQuery(MOBILE_BREAKPOINT);
  const { account } = useAccount();
  const [isCanceledModal, setCanceledModal] = useState(false);
  const [activeTab, setActiveTab] = useState('leaderboard');
  const [playGame, setPlayGame] = useState(false);

  const { tournamentGame, previousGame, setPreviousGame } = useGame();

  const [gameOver, setGameOver] = useAtom(GAME_OVER);
  const { configState } = useGame();
  const [coins, setCoins] = useAtom(COINS);

  const level = tournamentGame?.level || previousGame?.level;
  const score = configState && calculatePoints(coins, configState, level as Level);

  const stage = tournamentGame?.stage || previousGame?.stage;
  const isRegistration = Boolean(stage && 'registration' in stage);
  const isFinished = Boolean(stage && 'finished' in stage);
  const isStarted = Boolean(stage && 'started' in stage);

  useEffect(() => {
    const admin = tournamentGame?.admin || previousGame?.admin;
    const isAdmin = admin === account?.decodedAddress;

    if (previousGame && !tournamentGame) {
      setGameOver(false);
      if (!isAdmin) {
        setCanceledModal(true);
      } else {
        setPreviousGame(null);
      }
    }
  }, [account?.decodedAddress, previousGame, setGameOver, setPreviousGame, tournamentGame]);

  useEffect(() => {
    if (playGame || isStarted) {
      setActiveTab('play');
    }
  }, [isStarted, playGame, setActiveTab]);

  useEffect(() => {
    setCoins({ gold: 0, silver: 0 });
  }, [activeTab, setCoins]);

  return (
    <div className="grid gap-4 grid-cols-1 md:grid-cols-2">
      {isMobile && (
        <div>
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
                  isCanceledModal={isCanceledModal}
                  gameOver={gameOver}
                  score={score}
                />
              </TabsContent>
            </div>
          </Tabs>
        </div>
      )}

      <div className="hidden md:flex md:col-span-1 lg:col-span-1 md:py-5 bg-white rounded-md max-w-sm">
        {isRegistration && previousGame && <Registration tournamentGame={previousGame} />}
        {isStarted && <GamePlayers />}
      </div>

      {!isMobile && (
        <div className="hidden md:flex md:col-span-1 lg:col-span-1 p-5 bg-white rounded-md">
          <GameInfoCanvas
            isStarted={isStarted}
            isRegistration={isRegistration}
            isFinished={isFinished}
            isCanceledModal={isCanceledModal}
            gameOver={gameOver}
            score={score}
          />
        </div>
      )}

      {isFinished && tournamentGame && !isRegistration && <GameOverModal tournamentGame={tournamentGame} />}
      {isCanceledModal && <GameCanceledModal />}
    </div>
  );
};
