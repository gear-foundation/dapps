import React, { useEffect, useState } from 'react'
import { useAccount, useApi } from '@gear-js/react-hooks'
import { Button } from '@gear-js/vara-ui'

import { useGame } from '@/app/context/ctx-game'
import { cn } from '@/app/utils'
import { SpriteIcon } from '@/components/ui/sprite-icon'
import { useApp } from '@/app/context/ctx-app'
import { useGameMessage } from '@/app/hooks/use-game'
import { PRIZE_POOL } from '@/feature/game/consts'
import { useAtom } from 'jotai'

export const GamePlayers = () => {
	const { api } = useApi();
	const { account } = useAccount()
	const { tournamentGame } = useGame()
	const { isPending, setIsPending } = useApp()
	const [, setPrizePool] = useAtom(PRIZE_POOL)
	const handleMessage = useGameMessage();
	const [sortedParticipants, setSortedParticipants] = useState<any>([])

	const startTime = parseInt(tournamentGame?.[0].stage.Started.replace(/,/g, '') || "0", 10);
	const durationMs = parseInt(tournamentGame?.[0].durationMs.replace(/,/g, '') || "0", 10);
	const endTime = startTime + durationMs;

	const [timeLeft, setTimeLeft] = useState(endTime - Date.now());

	const onSuccess = () => setIsPending(false);

	const isAdmin = tournamentGame?.[0].admin === account?.decodedAddress

	const onCancelGame = () => {
		setIsPending(true)
		handleMessage({
			payload: { CancelTournament: null },
			onSuccess,
			onError: onSuccess,
		})
	}

	const [decimals] = api?.registry.chainDecimals ?? [12];
	const bid = parseFloat(String(tournamentGame?.[0].bid).replace(/,/g, '') || "0") / 10 ** decimals

	useEffect(() => {
		const updateTimer = () => {
			const now = Date.now();
			const timeLeft = endTime - now;
			setTimeLeft(Math.max(timeLeft, 0));
		};

		const timerId = setInterval(updateTimer, 1000);

		return () => clearInterval(timerId);
	}, [endTime]);

	const minutes = Math.floor(timeLeft / 60000); // 60000 миллисекунд в минуте
	const seconds = Math.floor((timeLeft % 60000) / 1000); // остаток от деления на 60000, переведенный в секунды

	const formattedTimeLeft = `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;

	useEffect(() => {
		const pool = bid * tournamentGame![0].participants.length

		setPrizePool(pool)
	}, [])

	useEffect(() => {
		const sortedParticipants = tournamentGame?.[0].participants
			.map(participant => {
				const timeInMs = parseInt(participant[1].time.replace(/,/g, ''), 10); // Преобразование времени из строки в число
				const points = parseInt(participant[1].points, 10); // Преобразование очков из строки в число
				return {
					address: participant[0],
					name: participant[1].name,
					timeInMs,
					points,
				};
			})
			.sort((a, b) => {
				if (a.timeInMs !== b.timeInMs) return a.timeInMs - b.timeInMs;
				return a.points - b.points;
			});

		setSortedParticipants(sortedParticipants)

	}, [tournamentGame])

	return (
		<div className="flex flex-col gap-4 items-center w-3/5">
			<h3 className="text-2xl font-bold">{tournamentGame?.[0].tournamentName}</h3>
			<div className="flex gap-10 justify-between">
				<div className="flex gap-3">
					<p className="text-[#555756]">Prize pool:</p>
					<div className="flex gap-3 font-semibold">
						<SpriteIcon name="vara-coin" height={24} width={24} />
						{bid * tournamentGame![0].participants.length}
					</div>
				</div>

				<div className="flex gap-3">
					<p className="text-[#555756]">Tournament ends:</p>
					<div className="flex gap-3 font-semibold">
						{formattedTimeLeft}
					</div>
				</div>
			</div>

			<div className="flex flex-col gap-3 w-full">
				{sortedParticipants?.map((participant: { address: React.Key | null | undefined; timeInMs: number; name: string | number | boolean | React.ReactElement<any, string | React.JSXElementConstructor<any>> | Iterable<React.ReactNode> | React.ReactPortal | null | undefined; points: string | number | boolean | React.ReactElement<any, string | React.JSXElementConstructor<any>> | Iterable<React.ReactNode> | React.ReactPortal | null | undefined }, index: any) => {
					const isActivePlayer = account?.decodedAddress === participant.address;
					const minutes = Math.floor(participant.timeInMs / 60000);
					const seconds = Math.floor((participant.timeInMs % 60000) / 1000);
					const timeFormatted = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;

					return (
						<div key={participant.address} className={cn(
							"flex items-center justify-between p-2 bg-white border border-[#EDEDED] rounded-lg",
							isActivePlayer && "bg-[#00FFC4] border-[#00EDB6]"
						)}>
							<div className="flex items-center gap-3">
								<p className="font-semibold">{participant.name}</p>
							</div>
							<div className="flex items-center justify-end gap-1 w-full mr-20">
								<p className="font-semibold">{timeFormatted}</p>
							</div>
							<div className="flex items-center gap-3">
								<p className="font-semibold">{participant.points}</p>
							</div>
						</div>
					);
				})}
			</div>
			<div className="flex gap-3 justify-between w-full">
				{isAdmin && <Button className="!bg-[#EB5757] !text-white !text-[14px] w-full" text="Cancel tournament" onClick={onCancelGame} isLoading={isPending} />}
			</div>
		</div>
	)
}
