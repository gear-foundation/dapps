
import { useEffect, useState } from 'react';
import { useAtom } from 'jotai'
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';
import { useAccount } from '@gear-js/react-hooks';

import { useGame } from '@/app/context/ctx-game';

import { Icons } from '@/components/ui/icons'
import { GameTimer } from './components/timer';
import { GameCanvas } from './GameCanvas'
import { Registration } from './components/registration';
import { GamePlayers } from './components/game-players';
import { GameOverModal } from './components/modals/game-over';
import { GameCanceledModal } from './components/modals/game-canceled';

import { calculatePoints } from '../game/utils/calculatePoints';
import { COINS, GAME_OVER } from '../game/consts';

import { IGameLevel } from '@/app/types/game';

export const Game = () => {
	const { account } = useAccount()
	const [isCanceledModal, setCanceledModal] = useState(false)

	const { tournamentGame, previousGame, setPreviousGame } = useGame()


	const [gameOver, setGameOver] = useAtom(GAME_OVER)
	const { configState } = useGame()
	const [coins] = useAtom(COINS);

	const level = tournamentGame?.[0].level || previousGame?.[0].level
	const score = configState && calculatePoints(coins, configState, level as IGameLevel)

	const isRegistration = tournamentGame?.[0].stage === "Registration" || previousGame?.[0].stage === "Registration"
	const isFinished = tournamentGame?.[0].stage.Finished || previousGame?.[0].stage.Finished
	const isStarted = tournamentGame?.[0].stage.Started || previousGame?.[0].stage.Started

	useEffect(() => {
		const admin = tournamentGame?.[0].admin || previousGame?.[0].admin
		const isAdmin = admin === account?.decodedAddress;

		if (previousGame && !tournamentGame) {
			if (!isAdmin) {
				setCanceledModal(true)
			} else {
				setPreviousGame(null)
			}
		}
	}, [tournamentGame])

	return (
		<div className="flex">
			{isRegistration && previousGame && <Registration tournamentGame={previousGame} />}
			{isStarted && <GamePlayers />}
			{isFinished && tournamentGame && <GameOverModal tournamentGame={tournamentGame} />}
			{isCanceledModal && <GameCanceledModal />}

			<div className="w-full flex flex-col justify-center items-center">
				<div className="w-[588px] flex justify-between my-3">
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

				<GameCanvas isPause={isRegistration || isFinished || !isStarted} />

				<div className="flex gap-5 my-3">
					<div className="flex gap-3 items-center">
						<div className="bg-[#DFDFDF] rounded-sm p-1">
							<ArrowUp color='#767676' />
						</div>

						<div className="bg-[#DFDFDF] rounded-sm p-1">
							<ArrowDown color='#767676' />
						</div>

						<span>Use arrows to move</span>
					</div>

					<div className="flex gap-3 items-center">
						<div className="bg-[#DFDFDF] rounded-sm p-1">
							<ArrowLeft color='#767676' />
						</div>

						<div className="bg-[#DFDFDF] rounded-sm p-1">
							<ArrowRight color='#767676' />
						</div>

						<span>Rotate</span>
					</div>

					<div className="flex gap-3 items-center">
						<div className="bg-[#DFDFDF] rounded-sm p-1 px-3 font-bold text-[#726F6F]">
							Shift
						</div>

						<span>Hold shift to run</span>
					</div>
				</div>

			</div>
		</div>
	)
}
