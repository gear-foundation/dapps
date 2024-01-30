import { Dialog } from '@headlessui/react';
import { Dispatch, SetStateAction } from 'react';
import { useApp, useGame } from '../../../app/context';
import { PopupContainer } from '../popup-container';
import { useGameMessage } from 'app/hooks/use-game';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const WinnerPopup = ({ setIsOpen, isOpen }: Props) => {
  const { game, isAdmin } = useGame();
  const { setIsPending } = useApp();
  const handleMessage = useGameMessage();

  const onSuccess = () => {
    setIsPending(false);
  };
  const onError = () => {
    setIsPending(false);
  };

  const onRestartGame = () => {
    handleMessage({
      payload: { RestartGame: null },
      onSuccess,
      onError,
    });
  }

  const winnerId = game?.gameState?.state?.Winner;
  const index = game?.players && winnerId ? game.gameState?.players.findIndex((id) => id[0] === winnerId[0]) : -1;

  return (
    <PopupContainer isOpen={isOpen} setIsOpen={setIsOpen} overlayCn="bg-black/90 backdrop-blur">
      <div className="flex items-center pt-40 w-full">
        <div className="relative w-full max-w-3xl transform transition-all">
          <img
            src="/images/winner.svg"
            alt="Winner"
            className="absolute bottom-[80%] left-1/2 -translate-x-1/2 h-[175%]"
          />
          <img
            src="/images/winner-bg.svg"
            alt="Winner"
            className="absolute bottom-[60%] -z-1 left-1/2 -translate-x-1/2 h-[200%]"
          />
          <div className="rounded-2xl bg-white px-8 py-12 xxl:pt-12 xxl:pb-19 border-6 border-[#1E942A] shadow-xl">
            <Dialog.Title
              as="h3"
              className="text-[40px] xxl:text-[48px] leading-18 font-bold text-center text-transparent-primary"
              id="modal-headline">
              Congrats!
            </Dialog.Title>
            <div className="mt-2">
              <Dialog.Description
                as="p"
                className="text-lg xxl:text-[21px] leading-5 mt-6 text-center text-dark-500 font-extrabold tracking-[0.08em]">
                <span className="text-[#00D1FF]">{game?.gameState?.players && index > 0 ? game?.gameState?.players[index][1] : 'Señor'}</span>{' '}
                is a winner! Take your tequila and enjoy!
              </Dialog.Description>
              <div className="flex gap-6 absolute bottom-0 left-1/2 mt-4 w-[355px] -translate-x-1/2 translate-y-1/2">
                <button
                  type="button"
                  tabIndex={0}
                  className="btn btn--primary w-full text-base xxl:text-xl text-dark-500 font-semibold"
                  onClick={() => setIsOpen(false)}>
                  OK
                </button>
                {isAdmin &&
                  <button
                    type="button"
                    tabIndex={0}
                    className="btn btn--white border-4 border-primary w-full text-base xxl:text-xl text-dark-500 font-semibold"
                    onClick={onRestartGame}>
                    Play again
                  </button>}
              </div>
            </div>
          </div>
        </div>
      </div>
    </PopupContainer>
  );
};
