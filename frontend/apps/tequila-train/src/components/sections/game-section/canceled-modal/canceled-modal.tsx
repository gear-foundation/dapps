import { Button } from '@gear-js/vara-ui';

import { useApp, useGame } from '@/app/context';
import { Modal } from '@/components/ui/modal';

export function CanceledSection() {
  const { setPreviousGame } = useGame();
  const { setOpenEmptyPopup } = useApp();

  const onLeaveGame = () => {
    setPreviousGame(null);
    setOpenEmptyPopup(false);
  };

  return (
    <Modal heading="The game has been canceled by the administrator" onClose={() => {}}>
      <p className="text-[#555756]">
        Game administrator has ended the game. All spent VARA tokens for the entry fee will be refunded.
      </p>

      <Button text="OK" color="grey" className="mt-5 w-64" onClick={onLeaveGame} />
    </Modal>
  );
}
