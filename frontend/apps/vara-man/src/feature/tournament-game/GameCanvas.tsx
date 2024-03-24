import { useRef, useEffect, useState } from 'react';
import { useAtom } from 'jotai'

import { TileMap } from '@/app/types/game';
import { useGameMessage } from '@/app/hooks/use-game';

import { findMapLevel } from '../game/utils/findMapLevel';
import { Game } from '../game/models/Game';
import { COINS, GAME_OVER, MS_TIME_GAME_OVER } from '../game/consts';
import { useGame } from '@/app/context/ctx-game';

import { GamePlayAgainModal } from './components/modals/game-play-again';

type Props = {
	isPause: boolean
}

export const GameCanvas = ({ isPause }: Props) => {
	const { tournamentGame, previousGame } = useGame()
	const [coins, setCoins] = useAtom(COINS)
	const [gameOver, setGameOver] = useAtom(GAME_OVER)
	const [timeGameOver] = useAtom(MS_TIME_GAME_OVER);
	const [messageSent, setMessageSent] = useState(false);
	const handleMessage = useGameMessage();
	const [isOpenPlayAgain, setIsOpenPlayAgain] = useState(false)

	const incrementCoins = (coinType: 'silver' | 'gold') => {
		setCoins((prevCoins) => ({
			...prevCoins,
			[coinType]: prevCoins[coinType] + 1
		}))
	}

	const level = tournamentGame?.[0].level || previousGame?.[0].level
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
			gameInstanceRef.current = new Game(gameContext, fogContext, level, incrementCoins, gameOver, setGameOver, map, isPause);
		}

		return () => {
			gameInstanceRef.current?.cleanup();
			mapRef.current = null;
		};

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [gameOver, level]);

	useEffect(() => {
		if (!isPause) {
			gameInstanceRef.current?.updatePause();
		}
	}, [isPause])

	useEffect(() => {
		gameInstanceRef.current?.updateGameOver(gameOver);

		if (!messageSent && gameOver && timeGameOver > 0) {
			setIsOpenPlayAgain(true);
			if (coins.gold > 0 || coins.silver > 0) {

				handleMessage({
					payload: {
						RecordTournamentResult: {
							time: timeGameOver,
							gold_coins: coins.gold,
							silver_coins: coins.silver,
						},
					},
				});

				setMessageSent(true);
			}
		}

		if (!gameOver) {
			setMessageSent(false);
		}

		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [gameOver, timeGameOver]);

	const restartGame = () => {
		setGameOver(false)
		setMessageSent(false);
		gameInstanceRef.current?.updateGameOver(gameOver);
		gameInstanceRef.current = null
	}

	return (
		<div className="ml-auto mr-auto">
			{isOpenPlayAgain && <GamePlayAgainModal setIsOpenPlayAgain={setIsOpenPlayAgain} restartGame={restartGame} />}
			<div className="ml-auto mr-auto" style={{ position: 'relative' }}>
				<canvas ref={fogCanvasRef} style={{ position: 'absolute', left: 0, top: 0 }} />
				<canvas ref={canvasRef} />
			</div>
		</div>
	)
};