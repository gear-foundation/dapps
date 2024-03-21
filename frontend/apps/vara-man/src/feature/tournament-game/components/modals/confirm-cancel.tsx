

import { Button } from '@gear-js/vara-ui';
import { Modal } from '@/components';
type Props = {
	setIsOpenCancelModal: (_: boolean) => void
	onCancelGame: () => void
}
export const ConfirmCancelModal = ({ setIsOpenCancelModal, onCancelGame }: Props) => {

	return (
		<Modal onClose={() => null}>
			<h2 className='typo-h2'>Sure you want to end the game?</h2>
			<div className="flex flex-col gap-5 mt-5">
				<p className="text-[#555756]">
					This action cannot be undone. The game will be concluded, and all players will exit the gaming room. Any entry fees will be refunded to all players.
				</p>
				<div className="flex gap-10">
					<Button color='grey' text="End tournament" onClick={onCancelGame} />
					<Button text="Continue tournament" onClick={() => setIsOpenCancelModal(false)} />
				</div>
			</div>
		</Modal>
	)
}
