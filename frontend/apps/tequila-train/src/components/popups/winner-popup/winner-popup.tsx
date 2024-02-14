import { Dialog } from '@headlessui/react';
import { Dispatch, SetStateAction } from 'react';
import { useApp, useGame } from '../../../app/context';
import { PopupContainer } from '../popup-container';
import { useGameMessage } from 'app/hooks/use-game';
import { playerNames } from 'app/consts';
import { Button } from '@gear-js/vara-ui';
import { PlayersGame } from 'app/types/game';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const WinnerPopup = ({ setIsOpen, isOpen }: Props) => {
  const { game, isAdmin, setPreviousGame } = useGame();
  const { setIsPending, setOpenWinnerPopup } = useApp();
  const handleMessage = useGameMessage();

  const onSuccess = () => {
    setIsPending(false);
    setPreviousGame(null)
  };
  const onError = () => {
    setIsPending(false);
  };

  const onLeaveGame = () => {
    setOpenWinnerPopup(false)

    handleMessage({
      payload: isAdmin ? { LeaveGame: null } : { CancelGame: null },
      onSuccess,
      onError,
    });
  }

  const winnerIds = game?.state?.Winners;
  const players = game?.gameState?.players;
  const winnerIndexes = winnerIds?.map(winnerId => players?.findIndex((player: PlayersGame) => player.id === winnerId)) || [];
  const winnerNames = winnerIndexes && winnerIndexes.map(index => playerNames[index || 0]).filter(name => name !== undefined);

  return (
    <PopupContainer isOpen={isOpen} setIsOpen={setIsOpen}>
      <div className="flex items-center pt-40 w-[600px]">
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
          <div className="rounded-2xl bg-white w-full px-6 py-15 xxl:pt-12 xxl:pb-19 shadow-xl">
            <Dialog.Title
              as="h3"
              className="text-[32px] xxl:text-[32px] font-bold text-black"
              id="modal-headline">
              Congrats!
            </Dialog.Title>
            <div className="mt-2">
              <Dialog.Description
                as="p"
                className="text-[14px] text-dark-500 font-semibold">
                {winnerNames.map((winnerName, index) => {
                  return (
                    <span key={index} className="text-[#00D1FF]">
                      {`Señor ${winnerName} `}
                    </span>
                  )
                })}

                is a winner! Take your tequila and enjoy!
              </Dialog.Description>
              <div className="flex gap-3 mt-5 w-full">
                <Button text='Close' color='grey' className="w-full" onClick={onLeaveGame} />
                {isAdmin && <Button text='Play again' className="w-full" onClick={onLeaveGame} />}
              </div>
            </div>
          </div>
        </div>
      </div>
    </PopupContainer>
  );
};
