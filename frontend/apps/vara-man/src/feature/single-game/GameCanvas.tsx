import { useRef, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useAtom } from 'jotai'

import { IGameLevel, TileMap } from '@/app/types/game';

import { GameOverModal } from './components/modals/game-over';
import { useGameMessage } from '@/app/hooks/use-game';
import { useApp } from '@/app/context/ctx-app';

import { findMapLevel } from '../game/utils/findMapLevel';
import { Game } from '../game/models/Game';
import { COINS, GAME_OVER } from '../game/consts';

export const GameCanvas = () => {
	const [searchParams] = useSearchParams()
	const [coins, setCoins] = useAtom(COINS)
	const [gameOver, setGameOver] = useAtom(GAME_OVER)
	const { setIsPending } = useApp()
	const handleMessage = useGameMessage();

	const incrementCoins = (coinType: 'silver' | 'gold') => {
		setCoins((prevCoins) => ({
			...prevCoins,
			[coinType]: prevCoins[coinType] + 1
		}))
	}

	const level = searchParams.get("level") as IGameLevel

	const fogCanvasRef = useRef<HTMLCanvasElement>(null);

	const canvasRef = useRef<HTMLCanvasElement>(null);
	const gameInstanceRef = useRef<Game | null>(null);
	const mapRef = useRef<TileMap | null>(null);

	useEffect(() => {

		if (canvasRef.current && fogCanvasRef.current && level && mapRef.current === null && gameInstanceRef.current === null) {
			const gameContext = canvasRef.current;
			const fogContext = fogCanvasRef.current;

			fogCanvasRef.current.width = canvasRef.current.width;
			fogCanvasRef.current.height = canvasRef.current.height;

			const map = findMapLevel(level);
			mapRef.current = map;
			gameInstanceRef.current = new Game(gameContext, fogContext, level, incrementCoins, gameOver, setGameOver, map);
		}

		return () => {
			gameInstanceRef.current?.cleanup();
			mapRef.current = null;
		};

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [gameOver, level]);

	useEffect(() => {
		gameInstanceRef.current?.updateGameOver(gameOver);

		if (gameOver && (coins.gold > 0 || coins.silver > 0)) {
			setIsPending(true)

			handleMessage({
				payload: {
					FinishSingleGame: {
						gold_coins: coins.gold,
						silver_coins: coins.silver,
					},
				},
				onSuccess: () => setIsPending(false),
				onError: () => setIsPending(false),
			});
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [gameOver]);

	const restartGame = () => {
		gameInstanceRef.current = null
	}

	return (
		<div className="ml-auto mr-auto">
			{gameOver && <GameOverModal restartGame={restartGame} />}
			<div className="ml-auto mr-auto" style={{ position: 'relative' }}>
				<canvas ref={fogCanvasRef} style={{ position: 'absolute', left: 0, top: 0 }} />
				<canvas ref={canvasRef} />
			</div>
		</div>
	)
};