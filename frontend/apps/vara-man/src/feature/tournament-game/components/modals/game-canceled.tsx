import { useGame } from '@/app/context/ctx-game';
import { Modal } from '@/components/ui/modal/modal2';
import { Button } from '@gear-js/vara-ui';

export const GameCanceledModal = () => {
  const { setPreviousGame } = useGame();
  return (
    <Modal open>
      <Modal.Content classNameContent="max-w-[700px]">
        <h2 className="typo-h2">The game has been canceled by the administrator</h2>
        <div className="flex flex-col gap-5 mt-5">
          <p className="text-[#555756]">
            Game administrator Samovit has ended the game. All spent VARA tokens for the entry fee will be refunded.
          </p>
          <div className="flex gap-10">
            <Button color="grey" text="OK" className="w-1/3" onClick={() => setPreviousGame(null)} />
          </div>
        </div>
      </Modal.Content>
    </Modal>
  );
};
