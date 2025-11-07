import { useEzTransactions } from 'gear-ez-transactions';
import { useAtom } from 'jotai';
import { useRef, useEffect, useState } from 'react';

import { useGame } from '@/app/context/ctx-game';
import { TileMap } from '@/app/types/game';
import { useRecordTournamentResultMessage } from '@/app/utils';
import useOnScreen from '@/hooks/use-on-screen';

import { GameCanvas } from '../game/components/game-canvas/game-canvas';
import { COINS, GAME_OVER, MS_TIME_GAME_OVER } from '../game/consts';
import { GameEngine } from '../game/models/Game';
import { findMapLevel } from '../game/utils/findMapLevel';

import { GamePlayAgainModal } from './components/modals/game-play-again';

type Props = {
  isPause: boolean;
  isCanceledModal: boolean;
};

export const GameLayout = ({ isPause, isCanceledModal }: Props) => {
  const { tournamentGame, previousGame } = useGame();
  const [coins, setCoins] = useAtom(COINS);
  const [gameOver, setGameOver] = useAtom(GAME_OVER);
  const [timeGameOver] = useAtom(MS_TIME_GAME_OVER);
  const [messageSent, setMessageSent] = useState(false);
  const { recordTournamentResultMessage } = useRecordTournamentResultMessage();
  const { gasless } = useEzTransactions();
  const [isOpenPlayAgain, setIsOpenPlayAgain] = useState(false);

  const incrementCoins = (coinType: 'silver' | 'gold') => {
    setCoins((prevCoins) => ({
      ...prevCoins,
      [coinType]: prevCoins[coinType] + 1,
    }));
  };

  const level = tournamentGame?.level || previousGame?.level;
  const fogCanvasRef = useRef<HTMLCanvasElement>(null);
  const isVisibleFog = useOnScreen(fogCanvasRef);

  const canvasRef = useRef<HTMLCanvasElement>(null);
  const gameInstanceRef = useRef<GameEngine | null>(null);
  const mapRef = useRef<TileMap | null>(null);

  useEffect(() => {
    if (
      canvasRef.current &&
      fogCanvasRef.current &&
      level &&
      mapRef.current === null &&
      gameInstanceRef.current === null &&
      isVisibleFog
    ) {
      const gameContext = canvasRef.current;
      const fogContext = fogCanvasRef.current;

      fogCanvasRef.current.width = canvasRef.current.width;
      fogCanvasRef.current.height = canvasRef.current.height;

      const map = findMapLevel(level);

      mapRef.current = map;
      gameInstanceRef.current = new GameEngine(
        gameContext,
        fogContext,
        level,
        incrementCoins,
        gameOver,
        setGameOver,
        map,
        isPause,
      );
    }

    return () => {
      gameInstanceRef.current?.cleanup();
      mapRef.current = null;
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gameOver, level, isVisibleFog]);

  useEffect(() => {
    if (!isPause) {
      gameInstanceRef.current?.updatePause();
    }
  }, [isPause]);

  useEffect(() => {
    // gameInstanceRef.current?.updateGameOver(gameOver);

    if (!messageSent && gameOver && timeGameOver > 0) {
      setIsOpenPlayAgain(true);
      if (coins.gold > 0 || coins.silver > 0) {
        if (!gasless.isLoading) {
          void recordTournamentResultMessage(timeGameOver, coins.gold, coins.silver, {});
          setMessageSent(true);
        }
      }
    }

    if (!gameOver) {
      setMessageSent(false);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gameOver, timeGameOver]);

  const restartGame = () => {
    setGameOver(false);
    setMessageSent(false);
    gameInstanceRef.current?.updateGameOver(gameOver);
    gameInstanceRef.current?.cleanup();
    gameInstanceRef.current = null;
    mapRef.current = null;
  };

  return (
    <div className="ml-auto mr-auto max-md:w-full z-2">
      {isOpenPlayAgain && !isCanceledModal && !isPause && (
        <GamePlayAgainModal setIsOpenPlayAgain={setIsOpenPlayAgain} restartGame={restartGame} />
      )}
      <GameCanvas
        canvasRef={canvasRef}
        fogCanvasRef={fogCanvasRef}
        gameInstanceRef={gameInstanceRef}
        isPause={isPause}
      />
    </div>
  );
};
