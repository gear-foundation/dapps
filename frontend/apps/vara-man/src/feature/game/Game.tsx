import { useAtom } from 'jotai'
import { ArrowUp, ArrowDown, ArrowLeft, ArrowRight } from 'lucide-react';

import { GameCanvas } from './GameCanvas'
import { Icons } from '@/components/ui/icons'
import { COINS, GAME_OVER } from './consts';
import { useGame } from '@/app/context/ctx-game';
import { GameTimer } from './components/timer';
import { calculatePoints } from './utils/calculatePoints';

export const Game = () => {
	const [, setGameOver] = useAtom(GAME_OVER)
	const { tournamentGame, configState, singleGame } = useGame()
	const [coins] = useAtom(COINS);

	const score = configState && singleGame && calculatePoints(coins, configState, singleGame?.[0].level)

	return (
		<div>
			{tournamentGame && <>Tournament</>}
			<div className="w-full flex flex-col justify-center items-center">
				<div className="w-[588px] flex justify-between my-3">
					<div className="flex gap-3 items-center">
						<div className="flex gap-3 items-center font-semibold">
							<Icons.statsTimer />

							<GameTimer />
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

				<GameCanvas />

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
