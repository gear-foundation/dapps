import { Button } from '@gear-js/vara-ui';

import { Modal } from '@/components';

type GameNotFoundModalProps = {
  setIsOpenFindModal: (_: boolean) => void;
};

export const GameNotFoundModal = ({ setIsOpenFindModal }: GameNotFoundModalProps) => {
  return (
    <Modal onClose={() => null}>
      <h2 className="typo-h2">Game not found</h2>
      <div className="flex flex-col gap-5 mt-5">
        <p className="text-[#555756]">
          Please check the entered address. It's possible the game has been canceled or does not exist.
        </p>
        <div className="flex gap-10">
          <Button color="grey" text="Cancel" className="w-full" onClick={() => setIsOpenFindModal(false)} />
        </div>
      </div>
    </Modal>
  );
};
