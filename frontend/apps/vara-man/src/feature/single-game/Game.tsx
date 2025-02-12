import { useEzTransactions } from 'gear-ez-transactions';
import { useAtom } from 'jotai';
import { useRef, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';

import { useApp } from '@/app/context/ctx-app';
import { TileMap } from '@/app/types/game';
import { Level, useFinishSingleGameMessage } from '@/app/utils';
import useOnScreen from '@/hooks/use-on-screen';

import { GameCanvas } from '../game/components/game-canvas/game-canvas';
import { COINS, GAME_OVER } from '../game/consts';
import { GameEngine } from '../game/models/Game';
import { findMapLevel } from '../game/utils/findMapLevel';

import { GameOverModal } from './components/modals/game-over';

export const Game = () => {
  const [searchParams] = useSearchParams();
  const [coins, setCoins] = useAtom(COINS);
  const [gameOver, setGameOver] = useAtom(GAME_OVER);
  const { setIsPending } = useApp();

  const { gasless } = useEzTransactions();

  const { finishSingleGameMessage } = useFinishSingleGameMessage();

  const incrementCoins = (coinType: 'silver' | 'gold') => {
    setCoins((prevCoins) => ({
      ...prevCoins,
      [coinType]: prevCoins[coinType] + 1,
    }));
  };

  const level = searchParams.get('level') as Level;

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
      isVisibleFog &&
      !gasless.isLoading
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
      );
    }

    return () => {
      gameInstanceRef.current?.cleanup();
      mapRef.current = null;
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gameOver, level, isVisibleFog, gasless.isLoading]);

  useEffect(() => {
    gameInstanceRef.current?.updateGameOver(gameOver);

    if (gameOver && (coins.gold > 0 || coins.silver > 0) && !gasless.isLoading) {
      setIsPending(true);
      finishSingleGameMessage(coins.gold, coins.silver, level, {
        onSuccess: () => setIsPending(false),
        onError: () => setIsPending(false),
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [gameOver, gasless]);

  const restartGame = () => {
    gameInstanceRef.current = null;
  };

  return (
    <div className="ml-auto mr-auto max-md:w-full z-10">
      {gameOver && <GameOverModal restartGame={restartGame} />}
      <GameCanvas gameInstanceRef={gameInstanceRef} canvasRef={canvasRef} fogCanvasRef={fogCanvasRef} />
    </div>
  );
};
