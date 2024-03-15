
import { useAtom } from 'jotai'

import { Button, Modal } from '@/components'
import { Icons } from '@/components/ui/icons';
import { useGame } from '@/app/context/ctx-game';

import { COINS, GAME_OVER } from '../../consts';
import { useGameMessage } from '@/app/hooks/use-game';
import { useApp } from '@/app/context/ctx-app';
import { calculatePoints } from '../../utils/calculatePoints';

export const GameOverModal = ({ restartGame }: any) => {
	const [, setGameOver] = useAtom(GAME_OVER)
	const [, setCoins] = useAtom(COINS)
	const { isPending, setIsPending } = useApp()
	const handleMessage = useGameMessage();

	const onSuccess = () => {
		setIsPending(false)
		setGameOver(false)
		setCoins({ gold: 0, silver: 0 })
		restartGame()
	};

	const [coins] = useAtom(COINS);
	const { configState, singleGame, tournamentGame } = useGame()
	const currentLevel = singleGame?.[0].level || tournamentGame?.level;

	const score = configState && singleGame && calculatePoints(coins, configState, singleGame?.[0].level)

	return (
		<div>
			<Modal onClose={() => null}>
				<div className="flex flex-col justify-center gap-5 text-center">
					<div>
						<h3 className='text-3xl font-semibold'>Game Over</h3>
						<p className="text-[#555756] mt-2">You're doing great, keep it up!</p>
					</div>
					<div className="bg-[#F7F9FA] w-full p-5 font-medium flex gap-5 justify-center items-center">
						Your score:
						<span className="flex items-center gap-2 font-semibold">
							<Icons.statsCoins />
							{score}
						</span>
					</div>
					<div className="flex justify-evenly">
						<Button variant='gray'
							onClick={() => {
								setIsPending(true)
								handleMessage({
									payload: { LeaveGame: null },
									onSuccess,
									onError: onSuccess,
								})
							}
							}
							isLoading={isPending}
							disabled={isPending}
						>
							Close
						</Button>
						<Button
							onClick={() => {
								setIsPending(true)
								handleMessage({
									payload: {
										StartSingleGame: { level: currentLevel },
									},
									onInBlock: () => setIsPending(true),
									onSuccess,
								})
							}
							}
							isLoading={isPending}
							disabled={isPending}
						>
							Play again
						</Button>
					</div>
				</div>
			</Modal>
		</div>
	)
}
