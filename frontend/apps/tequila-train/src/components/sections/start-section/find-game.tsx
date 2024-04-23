import { Button, Input } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useApi } from '@gear-js/react-hooks';
import { decodeAddress } from '@gear-js/api';

import styles from './start-secrtion.module.scss'
import { useGame, useApp } from 'app/context';
import { useGameMessage } from 'app/hooks/use-game';
import { useForm } from '@mantine/form';
import { stringRequired } from 'app/utils';
import { Modal } from 'components/ui/modal';
import { Icon } from 'components/ui/icon';
import { GameType } from 'app/types/game';

const initialValues = {
	creator: '',
};

const validate: Record<string, typeof stringRequired> = {
	creator: stringRequired,
};

export const FindGame = ({ closeFindGame }: { closeFindGame: () => void }) => {
	const { api } = useApi()
	const [isOpenModal, setOpenModal] = useState(false)
	const [findGame, setFindGame] = useState<GameType | null>(null)
	const { state, setPreviousGame } = useGame();
	const { setIsPending, isPending } = useApp();
	const handleMessage = useGameMessage();
	const [isNotFound, setIsNotFound] = useState(false)

	const form = useForm({
		initialValues,
		validate,
		validateInputOnChange: true,
	});
	const { getInputProps } = form;

	const onSuccess = () => {
		setIsPending(false);
	};
	const onError = () => {
		setIsPending(false);
	};

	const handleSubmit = form.onSubmit((values) => {
		const [decimals] = api?.registry.chainDecimals ?? [12];

		if (findGame) {
			setIsPending(true);
			handleMessage({
				payload: { Register: { creator: decodeAddress(values.creator) } },
				value: parseFloat(findGame.bid) * 10 ** decimals,
				onSuccess,
				onError,
			});
		}
	});

	const onFindGame = () => {
		setPreviousGame(null)
		const findGame = state?.games.find(game => game[0] === form.values.creator || game[0] === decodeAddress(form.values.creator))
		if (findGame) {
			setFindGame(findGame[1] as GameType)
			setOpenModal(true)
		} else {
			setIsNotFound(true)
		}
	}

	const [decimals] = api?.registry.chainDecimals ?? [12];
	const bid = parseFloat(findGame?.bid.replace(/,/g, '') || "0") / 10 ** decimals

	return (
		<div className="basis-[540px] grow lg:grow-0">
			<h2 className="text-[32px] leading-none font-bold text-black">Welcome to Tequila Train </h2>
			<p className="mt-3 text-[#555756]">
				To join the game, specify the address received from the game administrator.
			</p>
			<div className="mt-6">
				<Input
					type="text"
					label="Specify the game admin address:"
					placeholder="kG…"
					{...getInputProps('creator')}
				/>
			</div>

			<div className="mt-6 flex gap-5">
				<Button text="Continue" color="primary" className={styles.connectButton} onClick={onFindGame} disabled={!form.values.creator} />
				<Button text="Cancel" color="grey" className={styles.connectButton} onClick={closeFindGame} />
			</div>

			{isOpenModal &&
				<Modal heading="The game has been found" onClose={() => setOpenModal(false)}>
					<p>To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will need to pay the entry fee immediately after clicking the “Join” button.</p>
					<div className="bg-[#F7F9FA] rounded-2xl text-black my-5 p-4">
						<div className="flex flex-col gap-2">
							<div className="flex items-center justify-between pr-[100px]">
								<p>Entry fee</p>
								<div className="flex items-center font-semibold">
									<Icon name='vara-coin' width={24} height={24} className="mr-2" />
									{bid} VARA
								</div>
							</div>

							<div className="flex items-center justify-between pr-[100px]">
								<p>Players already joined the game</p>
								<div className="flex items-center gap-1">
									<Icon name="user" width={24} height={24} />
									<span className="font-semibold">{findGame?.initialPlayers.length} </span>
									/8</div>
							</div>
						</div>
					</div>
					<form onSubmit={handleSubmit}>
						<div className="w-full flex gap-5">
							<Button text="Cancel" color="grey" className="w-full" onClick={() => setOpenModal(false)} />
							<Button text="Join" color="primary" className="w-full" onClick={onFindGame} type="submit" disabled={isPending} />
						</div>
					</form>
				</Modal>
			}

			{isNotFound &&
				<Modal heading="Game not found" onClose={() => setIsNotFound(false)}>
					<p>Please check the entered address. It's possible the game has been canceled or does not exist.</p>
					<Button text="OK" color="grey" className="w-72 mt-5" onClick={() => setIsNotFound(false)} />
				</Modal>
			}
		</div >
	);
};
