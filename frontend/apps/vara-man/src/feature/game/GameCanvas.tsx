import { useRef, useEffect } from 'react';
import { useAtom } from 'jotai'

import { Game } from './models/Game';
import { IGameLevel } from '@/app/types/game'
	;
import { COINS, GAME_OVER } from './consts';
import { useGame } from '@/app/context/ctx-game';
import { GameOverModal } from './components/modals/game-over';
import { findMapLevel } from './utils/findMapLevel';
import { TileMap } from './types';
import { useGameMessage } from '@/app/hooks/use-game';
import { useApp } from '@/app/context/ctx-app';

export const GameCanvas = () => {
	const [coins, setCoins] = useAtom(COINS)
	const { singleGame } = useGame()
	const [gameOver, setGameOver] = useAtom(GAME_OVER)
	const { setIsPending } = useApp()
	const handleMessage = useGameMessage();

	const incrementCoins = (coinType: 'silver' | 'gold') => {
		setCoins((prevCoins) => ({
			...prevCoins,
			[coinType]: prevCoins[coinType] + 1
		}))
	}

	const level: IGameLevel = singleGame?.[0].level as IGameLevel

	const canvasRef = useRef<HTMLCanvasElement>(null);
	const gameInstanceRef = useRef<Game | null>(null);
	const mapRef = useRef<TileMap | null>(null);

	useEffect(() => {
		if (canvasRef.current && level && mapRef.current === null && gameInstanceRef.current === null) {
			const map = findMapLevel(level);
			mapRef.current = map;
			gameInstanceRef.current = new Game(canvasRef.current, level, incrementCoins, gameOver, setGameOver, map);
		}

		return () => {
			gameInstanceRef.current?.cleanup();
			mapRef.current = null;
		};

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [gameOver, level, singleGame]);

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
			<canvas ref={canvasRef} id="game" />
		</div>
	)
};
