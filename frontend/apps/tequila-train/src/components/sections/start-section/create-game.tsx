import { Button, Input } from '@gear-js/vara-ui';

import styles from './start-secrtion.module.scss'
import { Sprite } from 'components/ui/sprite';
import { useForm } from '@mantine/form';
import { useApp } from 'app/context';
import { useGameMessage } from 'app/hooks/use-game';
import { numberRequired } from 'app/utils';
import { useApi } from '@gear-js/react-hooks';

const initialValues = {
	bid: 0,
};


export const CreateGame = ({ closeCreateGame }: { closeCreateGame: () => void }) => {
	const { api } = useApi();

	const { setIsPending, isPending } = useApp();
	const form = useForm({
		initialValues,
		validateInputOnChange: true,
	});
	const { getInputProps, errors, reset } = form;

	const handleMessage = useGameMessage();
	const onSuccess = () => {
		setIsPending(false);
		reset();
	};
	const onError = () => {
		setIsPending(false);
	};

	const handleSubmit = form.onSubmit((values) => {
		const [decimals] = api?.registry.chainDecimals ?? [12];
		setIsPending(true);
		handleMessage({
			payload: { CreateGame: null }, value: (values.bid * 10 ** decimals).toString() || "0",
			onSuccess,
			onError,
		});
	});

	return (
		<div className="container my-15 py-32 flex items-center px-50 ">
			<div className="grow flex space-x-8 justify-between bg-white pr-20 pl-11 py-19 min-h-[330px] rounded-[32px] text-white">
				<div className="basis-[540px] grow lg:grow-0">
					<h2 className="text-[32px] leading-none font-bold text-black">Create new game</h2>
					<p className="mt-3 text-[#555756]">
						Set the entry fee. After creating the game, share your unique game ID (which is your wallet address) so players can join.
					</p>
					<form onSubmit={handleSubmit}>
						<div className="mt-6">
							<Input
								type="number"
								min={0}
								label="Entry fee"
								icon={() => <Sprite name="vara-coin" height={24} width={24} />}
								{...getInputProps('bid')}
								required
							/>
						</div>

						<div className="mt-6 flex gap-5">
							<Button text="Create" color="primary" className={styles.connectButton} type="submit" disabled={isPending} />
							<Button text="Cancel" color="grey" className={styles.connectButton} onClick={closeCreateGame} />
						</div>
					</form>
				</div>
			</div>
		</div>
	);
};
