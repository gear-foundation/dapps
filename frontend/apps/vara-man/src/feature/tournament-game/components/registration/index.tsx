import React from 'react'
import { useGame } from '@/app/context/ctx-game'
import { cn, copyToClipboard, prettyAddress } from '@/app/utils'
import { SpriteIcon } from '@/components/ui/sprite-icon'
import { useAccount, useAlert, useApi } from '@gear-js/react-hooks'
import { Button } from '@gear-js/vara-ui'
import { useGameMessage } from '@/app/hooks/use-game'
import { useApp } from '@/app/context/ctx-app'
import { ITournamentGameInstance } from '@/app/types/game'

type Props = {
	tournamentGame: ITournamentGameInstance
}

export const Registration = ({ tournamentGame }: Props) => {
	const alert = useAlert();
	const { api } = useApi();
	const { account } = useAccount()
	const { setPreviousGame, setTournamentGame } = useGame()
	const { isPending, setIsPending } = useApp()
	const handleMessage = useGameMessage();

	const onSuccess = () => {

		setIsPending(false);
	}

	const isAdmin = tournamentGame?.[0].admin === account?.decodedAddress

	const onRemovePlayer = (player: string) => {
		setIsPending(true)
		handleMessage({
			payload: { DeletePlayer: player },
			onSuccess,
			onError: onSuccess,
		})
	}

	const onStartGame = () => {
		setIsPending(true)
		handleMessage({
			payload: { StartTournament: null },
			onSuccess,
			onError: onSuccess,
		})
	}

	const onCancelGame = () => {
		setIsPending(true)

		if (isAdmin) {
			handleMessage({
				payload: { CancelTournament: null },
				onSuccess,
				onError: onSuccess,
			})
		} else {
			handleMessage({
				payload: { CancelRegister: null },
				onSuccess,
				onError: onSuccess,
			})

			setPreviousGame(null)
			setTournamentGame(undefined)
		}
	}
	const [decimals] = api?.registry.chainDecimals ?? [12];
	const bid = parseFloat(String(tournamentGame?.[0].bid).replace(/,/g, '') || "0") / 10 ** decimals
	return (
		<div className="flex flex-col gap-4 items-center w-3/5">
			<h3 className="text-2xl font-bold">{tournamentGame?.[0].stage}</h3>
			<p className="text-[#555756]">Players ({tournamentGame?.[0].participants.length}/10). Waiting for other players... </p>
			{isAdmin &&
				<div className="flex gap-2 font-medium">
					Share the game's address:
					<span className="font-bold">
						({prettyAddress(account.address)})
					</span>

					<span className="font-semibold text-[#0ED3A3] cursor-pointer" onClick={() => copyToClipboard({ key: account.address, alert })}>Copy</span>
				</div>
			}
			<div className="flex flex-col gap-3 w-full">
				{tournamentGame?.[0].participants.map((player, index) => {
					const isActivePlayer = account?.decodedAddress === player[0]
					const { name, points, time } = player[1]

					return (
						<div key={player[0]} className={cn(
							"flex items-center justify-between p-2 bg-white border border-[#EDEDED] rounded-lg",
							isActivePlayer && "bg-[#00FFC4] border-[#00EDB6]"
						)}>
							<div className="flex items-center gap-3">
								{isAdmin && !isActivePlayer &&
									<button onClick={() => onRemovePlayer(player[0])}>
										<SpriteIcon name="close-gray" height={24} width={24} />
									</button>

								}

								{isAdmin && isActivePlayer &&
									<div className="py-2 px-3"></div>
								}

								{!isAdmin &&
									<div className="bg-[#F5F5F5] font-semibold py-2 px-5 rounded">
										{index + 1}
									</div>
								}

								<p className="font-semibold">{name}</p>
							</div>
							<div className="flex items-center gap-3">
								<SpriteIcon name="vara-coin" height={24} width={24} />
								<p className="font-semibold">{bid}</p>
							</div>
						</div>
					)
				})}
			</div>

			<div className="flex gap-3 justify-between w-full">
				{isAdmin ?
					<>
						<Button className="!bg-[#EB5757] !text-white !text-[14px]" text="Cancel tournament" onClick={onCancelGame} isLoading={isPending} />
						<Button className="!text-[14px]" text="Start tournament" onClick={onStartGame} isLoading={isPending} />
					</> :

					<Button className="!text-[14px] w-full" color='grey' text="Cancel" onClick={onCancelGame} isLoading={isPending} />
				}
			</div>
		</div>
	)
}
