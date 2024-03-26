

import { useApp } from '@/app/context/ctx-app';
import { useGameMessage } from '@/app/hooks/use-game';
import { Modal } from '@/components';
import { SpriteIcon } from '@/components/ui/sprite-icon';
import { useApi } from '@gear-js/react-hooks';
import { Input, Button } from '@gear-js/vara-ui';
import { useState } from 'react';

type GameFindModalProps = {
	findGame: {
		admin: string,
		bid: bigint,
		participants: number
	}
	setIsOpenFindModal: (_: boolean) => void
}

export const GameFindModal = ({ findGame, setIsOpenFindModal }: GameFindModalProps) => {
	const [username, setUsername] = useState("")

	const { api } = useApi();
	const { isPending, setIsPending } = useApp();
	const handleMessage = useGameMessage();

	const onSuccess = () => {
		setIsPending(false);
	};
	const onError = () => {
		setIsPending(false);
	};

	const [decimals] = api?.registry.chainDecimals ?? [12];
	const bid = parseFloat(String(findGame?.bid).replace(/,/g, '') || "0") / 10 ** decimals

	const onJoinGame = () => {
		if (username) {
			setIsPending(true);
			handleMessage({
				payload: {
					RegisterForTournament: {
						admin_id: findGame.admin,
						name: username
					}
				},
				value: (bid * 10 ** decimals).toString() || "0",
				onSuccess,
				onError,
			});
		}
	}

	return (
		<Modal onClose={() => null}>
			<h2 className='typo-h2' > The game has been found</h2>
			<div className="flex flex-col gap-5 mt-5">
				<p className="text-[#555756]">
					To proceed, review the parameters of the gaming session and click the “Join” button.
					If applicable, you will need to pay the entry fee and required amount of gas immediately after clicking the “Join” button.
					After the end of the game, any unused gas will be refunded.
				</p>

				<div className="bg-[#f0f2f3] rounded-2xl text-black p-4">
					<div className="flex flex-col gap-2">
						<div className="flex items-center justify-between pr-[100px]">
							<p>Entry fee</p>
							<div className="font-semibold flex items-center">
								<SpriteIcon name='vara-coin' width={24} height={24} className="mr-2" />
								{bid} VARA
							</div>
						</div>

						<div className="flex items-center justify-between pr-[100px]">
							<p>Players already joined the game</p>
							<div className="font-semibold flex items-center">
								<SpriteIcon name='user' width={24} height={24} className="mr-2" />
								<span className="font-semibold">{findGame.participants} </span>
								/10
							</div>
						</div>

					</div>
				</div>

				<Input
					type="text"
					label='Enter your name:'
					placeholder='Username'
					required
					className="w-full"
					onChange={(e) => setUsername(e.target.value)}
				/>

				<div className="flex gap-10">
					<Button color='grey' text="Cancel" className="w-full" onClick={() => setIsOpenFindModal(false)} />
					<Button text="Join" className="w-full" onClick={onJoinGame} isLoading={isPending} />
				</div>
			</div>
		</Modal>
	)
}
