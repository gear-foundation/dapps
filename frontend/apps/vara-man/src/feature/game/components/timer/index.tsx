import { useEffect, useState } from 'react'
import { useAtom } from 'jotai';

import { useGame } from '@/app/context/ctx-game';
import { GAME_OVER } from '../../consts';

export const GameTimer = () => {
	const { singleGame, configState } = useGame()
	const [, setGameOver] = useAtom(GAME_OVER)

	const [timeLeft, setTimeLeft] = useState('');

	const currentTime = singleGame?.[1]
	const startTime = singleGame?.[0].startTime

	useEffect(() => {
		if (startTime && currentTime) {
			const totalGameTime = Number(configState?.timeForSingleRound.replace(/,/g, ''));
			const startTimeNumber = Number(startTime.replace(/,/g, ''));
			const currentTimeNumber = Number(currentTime.replace(/,/g, ''));

			const updateTimer = () => {
				const now = Date.now();
				const timePassedSinceLastUpdate = now - currentTimeNumber;
				const elapsedTime = currentTimeNumber - startTimeNumber + timePassedSinceLastUpdate;
				const remainingTime = totalGameTime - elapsedTime;

				if (remainingTime <= 0) {
					setTimeLeft(`00:00`);
					setGameOver(true)
				} else {
					const minutes = Math.floor((remainingTime / (1000 * 60)) % 60);
					const seconds = Math.floor((remainingTime / 1000) % 60);
					setTimeLeft(`${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`);
				}
			};

			const timerId = setInterval(updateTimer, 1000);

			return () => clearInterval(timerId);
		}
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [configState, startTime, currentTime]);

	return (
		timeLeft
	)
}
