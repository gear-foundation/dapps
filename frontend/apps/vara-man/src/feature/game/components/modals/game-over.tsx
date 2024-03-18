
import { useAtom } from 'jotai'

import { Button, Modal } from '@/components'
import { Icons } from '@/components/ui/icons';
import { useGame } from '@/app/context/ctx-game';

import { COINS, GAME_OVER } from '../../consts';
import { useApp } from '@/app/context/ctx-app';
import { calculatePoints } from '../../utils/calculatePoints';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { IGameLevel } from '@/app/types/game';

export const GameOverModal = ({ restartGame }: any) => {
	const [searchParams] = useSearchParams()
	const navigate = useNavigate()

	const [, setGameOver] = useAtom(GAME_OVER)
	const [, setCoins] = useAtom(COINS)
	const { isPending } = useApp()

	const [coins] = useAtom(COINS);
	const { configState } = useGame()
	const currentLevel = searchParams.get("level") as IGameLevel

	const score = configState && calculatePoints(coins, configState, currentLevel)

	const onResetGame = () => {
		setGameOver(false)
		setCoins({ gold: 0, silver: 0 })
		restartGame()
	}

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
								onResetGame()
								navigate("/")
							}}
							isLoading={isPending}
							disabled={isPending}
						>
							Close
						</Button>
						<Button
							onClick={() => onResetGame()}
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
