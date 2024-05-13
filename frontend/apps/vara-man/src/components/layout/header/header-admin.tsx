import { Icons } from '@/components/ui/icons'
import { useGame } from '@/app/context/ctx-game'
import { useGameMessage } from '@/app/hooks/use-game'
import { useApp } from '@/app/context/ctx-app'
import { useCheckBalance } from '@dapps-frontend/hooks'
import { useEzTransactions } from '@dapps-frontend/ez-transactions'

type HeaderAdminProps = BaseComponentProps & {}

export function HeaderAdmin({}: HeaderAdminProps) {
	const { isPending, setIsPending } = useApp()
	const { status } = useGame()

	const { gasless, signless } = useEzTransactions()
	const handleMessage = useGameMessage()
	const { checkBalance } = useCheckBalance({
		signlessPairVoucherId: signless.voucher?.id,
		gaslessVoucherId: gasless.voucherId,
	})
	const gasLimit = 120000000000

	const onSuccess = () => setIsPending(false)

	const onActivateGame = () => {
		if (!gasless.isLoading) {
			checkBalance(gasLimit, () =>
				handleMessage({
					payload: { ChangeStatus: { Started: null } },
					voucherId: gasless.voucherId,
					gasLimit,
					onSuccess,
					onError: onSuccess,
				})
			)
		}
	}

	const onDeactivateGame = () => {
		if (!gasless.isLoading) {
			checkBalance(gasLimit, () =>
				handleMessage({
					payload: { ChangeStatus: { Paused: null } },
					voucherId: gasless.voucherId,
					gasLimit,
					onSuccess,
					onError: onSuccess,
				})
			)
		}
	}

	return (
		<>
			{status === 'Paused' && (
				<button
					type="button"
					className="btn btn--primary px-6"
					disabled={isPending}
					onClick={onActivateGame}
				>
					<Icons.gameJoystick className="w-5 h-5 mr-2" />
					<span>Activate game</span>
				</button>
			)}
			{status === 'Started' && (
				<button
					type="button"
					className="btn btn--theme-red px-6"
					disabled={isPending}
					onClick={onDeactivateGame}
				>
					<Icons.gameJoystick className="w-5 h-5 mr-2" />
					<span>Deactivate game</span>
				</button>
			)}
		</>
	)
}
