import { Button, Input } from '@gear-js/vara-ui';
import { useState } from 'react';

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
	const [isOpenModal, setOpenModal] = useState(false)
	const [findGame, setFindGame] = useState<GameType | null>(null)
	const { state, setPreviousGame } = useGame();
	const { setIsPending, isPending } = useApp();
	const handleMessage = useGameMessage();

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
		setPreviousGame(null)
		setIsPending(true);
		handleMessage({
			payload: { Register: { creator: values.creator } },
			onSuccess,
			onError,
		});
	});

	const onFindGame = () => {
		const findGame = state?.games.find(game => game[0] === form.values.creator)
		if (findGame) {
			setFindGame(findGame[1] as GameType)
			setOpenModal(true)
		}
	}

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
					placeholder="0x25c..."
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
									{findGame?.bid} VARA
								</div>
							</div>

							<div className="flex items-center justify-between pr-[100px]">
								<p>Players already joined the game</p>
								<div className="flex items-center gap-1">
									<Icon name="user" width={24} height={24} />
									<span className="font-semibold">{findGame?.initialPlayers.length} </span>
									/8</div>
							</div>

							{/* <div className="flex items-center justify-between pr-[100px]">
								<p>Your game address
									<span className="font-bold"> ({account && shortenString(account.address, 4)})</span>
								</p>
								<div className="cursor-pointer text-[#0ED3A3] font-semibold" onClick={onCopy}>
									<Icon name='copy' width={24} height={24} className="mr-2" />
									Copy
								</div>
							</div> */}
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
		</div>
	);
};
