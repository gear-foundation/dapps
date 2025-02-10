import { useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Dialog } from '@headlessui/react';
import { Dispatch, SetStateAction } from 'react';

import { playerNames } from '@/app/consts';
import { useGameMessage } from '@/app/hooks/use-game';
import { PlayersGame } from '@/app/types/game';
import { Icon } from '@/components/ui/icon';

import { useApp, useGame } from '../../../app/context';
import { PopupContainer } from '../popup-container';

type Props = {
  setIsOpen: Dispatch<SetStateAction<boolean>>;
  isOpen: boolean;
};

export const WinnerPopup = ({ setIsOpen, isOpen }: Props) => {
  const { api } = useApi();
  const { game, isAdmin } = useGame();
  const { setIsPending, setOpenWinnerPopup, setIsUserCancelled } = useApp();
  const handleMessage = useGameMessage();

  const onSuccess = () => {
    setOpenWinnerPopup(false);
    setIsPending(false);
  };
  const onError = () => {
    setIsPending(false);
  };

  const onLeaveGame = () => {
    setIsUserCancelled(true);
    handleMessage({
      payload: isAdmin ? { CancelGame: null } : { LeaveGame: null },
      onSuccess,
      onError,
    });
  };

  const winnerIds = game?.state?.Winners;
  const players = game?.gameState?.players;
  const winnerIndexes =
    winnerIds?.map((winnerId) => players?.findIndex((player: PlayersGame) => player.id === winnerId)) || [];
  const winnerNames =
    winnerIndexes && winnerIndexes.map((index) => playerNames[index || 0]).filter((name) => name !== undefined);

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = parseFloat(game?.bid.replace(/,/g, '') || '0') / 10 ** decimals;

  return (
    <PopupContainer isOpen={isOpen} setIsOpen={setIsOpen}>
      <div className="flex items-center pt-96 w-[600px]">
        <div className="relative w-full max-w-3xl transform transition-all">
          <img
            src="/images/winner.svg"
            alt="Winner"
            className="absolute bottom-[90%] left-1/2 -translate-x-1/2 h-[150%]"
          />
          <img
            src="/images/winner-bg.svg"
            alt="Winner"
            className="absolute bottom-[60%] -z-1 left-1/2 -translate-x-1/2 h-[200%]"
          />
          <div className="rounded-2xl bg-white w-full px-6 py-15 xxl:pt-12 xxl:pb-19 shadow-xl">
            <Dialog.Title as="h3" className="text-[32px] xxl:text-[32px] font-bold text-black" id="modal-headline">
              Congrats!
            </Dialog.Title>
            <div className="mt-2">
              <Dialog.Description as="p" className="text-[14px] text-dark-500 font-semibold">
                {winnerNames.map((winnerName, index) => {
                  return (
                    <span key={index} className="text-[#00D1FF]">
                      {`Señor ${winnerName} `}
                    </span>
                  );
                })}
                is a winner! Take your tequila and enjoy!
                <div className="bg-[#F7F9FA] rounded-2xl flex items-center gap-5 p-5 mt-5 text-black">
                  <p>Winner's prize:</p>
                  <p className="font-semibold flex items-center gap-2 ">
                    <Icon name="vara-coin" width={24} height={24} />
                    {bid} VARA
                  </p>
                </div>
              </Dialog.Description>
              <div className="flex gap-3 mt-5 w-full">
                <Button text="Close" color="grey" className="w-full" onClick={onLeaveGame} />
                {isAdmin && <Button text="Play again" className="w-full" onClick={onLeaveGame} />}
              </div>
            </div>
          </div>
        </div>
      </div>
    </PopupContainer>
  );
};
