import { getVaraAddress, useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { useGameMessage } from '@/app/hooks/use-game';
import { Button } from '@gear-js/vara-ui';

import { cn, copyToClipboard, shortenString } from '@/app/utils';

import { useApp, useGame } from '@/app/context';
import { HexString } from '@gear-js/api';

import { Icon } from '@/components/ui/icon';
import { Modal } from './modal';

import { MockGameSection } from '../game-section/mock/mock-game-section';

export function RegistrationSection() {
  const { api } = useApi();
  const { account } = useAccount();
  const { game, isAdmin } = useGame();
  const { setIsPending, setIsUserCancelled } = useApp();
  const handleMessage = useGameMessage();
  const alert = useAlert();

  const onSuccess = () => {
    setIsPending(false);
  };
  const onError = () => {
    setIsPending(false);
  };

  const onStartGame = () => {
    handleMessage({
      payload: { StartGame: null },
      onSuccess,
      onError,
    });
  };

  const onCancelGame = () => {
    if (isAdmin) {
      handleMessage({
        payload: { CancelGame: null },
        onSuccess,
        onError,
      });
    } else {
      setIsUserCancelled(true);
      handleMessage({
        payload: { CancelRegistration: { creator: game?.admin } },
        onSuccess,
        onError,
      });
    }
  };

  const onDeletePlayer = (player: HexString) => {
    handleMessage({
      payload: { DeletePlayer: { player_id: player } },
      onSuccess,
      onError,
    });
  };

  const onCopy = () => {
    if (account) {
      copyToClipboard(account.address, alert);
    }
  };

  const reversedArrayPlayers = [...(game?.initialPlayers ?? [])];
  const disableButton = !Boolean(game && game.initialPlayers?.length >= 2);

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = parseFloat(game?.bid.replace(/,/g, '') || '0') / 10 ** decimals;

  return (
    <div>
      <MockGameSection />

      <Modal>
        <div className="container my-15 py-32 flex items-center">
          <div className="grow flex space-x-8 justify-between bg-white pr-20 pl-11 py-19 min-h-[330px] rounded-[32px] text-white font-kanit">
            <div className="relative basis-[220px] lg:basis-[365px] grow-0 shrink-0">
              <div className="absolute -inset-y-10 lg:-top-52 lg:-bottom-21.5 inset-x-0">
                <img
                  width={733}
                  height={955}
                  className="h-full w-full object-contain"
                  src="/images/register.webp"
                  alt="image"
                  loading="lazy"
                />
              </div>
            </div>
            <div className="basis-[540px] grow lg:grow-0">
              <h2 className="text-[32px] leading-none font-bold text-black">Registration...</h2>
              <p className="mt-3 text-[#555756]">
                Players ({game?.initialPlayers.length || 0}/8). Waiting for other players...{' '}
              </p>

              <div className="mt-6">
                {isAdmin && (
                  <div>
                    <div className="bg-[#F7F9FA] rounded-2xl text-black p-4">
                      <div className="flex flex-col gap-2">
                        <div className="flex items-center justify-between pr-[100px]">
                          <p>Entry fee</p>
                          <div className="font-semibold flex items-center">
                            <Icon name="vara-coin" width={24} height={24} className="mr-2" />
                            {bid} VARA
                          </div>
                        </div>

                        <div className="flex items-center justify-between pr-[100px]">
                          <p>Players already joined the game</p>
                          <div className="font-semibold flex items-center">
                            <span className="font-semibold">{game?.initialPlayers.length} </span>
                            /8
                          </div>
                        </div>

                        <div className="flex items-center justify-between pr-[100px]">
                          <p>
                            Your game address
                            <span className="font-bold"> ({account && shortenString(account.address, 4)})</span>
                          </p>
                          <div
                            className="cursor-pointer text-[#0ED3A3] font-semibold flex items-center"
                            onClick={onCopy}>
                            <Icon name="copy" width={24} height={24} className="mr-2" />
                            Copy
                          </div>
                        </div>
                      </div>
                    </div>

                    <div className="flex flex-col gap-1 my-5">
                      {reversedArrayPlayers.map((player, i) => {
                        return (
                          <div
                            key={player}
                            className={cn(
                              'py-2 px-6 rounded-2xl border-solid border border-[#EDEDED] text-black flex items-center justify-between',
                              i === 0 ? 'border-[#00FFC4]' : 'border-[#EDEDED]',
                            )}>
                            <p className="font-semibold">
                              {account && shortenString(getVaraAddress(player), 4)}
                              {i === 0 && <span className="text-[#00FFC4]"> (you)</span>}
                            </p>

                            {i !== 0 && (
                              <button
                                className="p-0 bg-transparent border-none cursor-pointer"
                                onClick={() => onDeletePlayer(player)}>
                                <Icon name="delete" height={24} width={24} />
                              </button>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}

                <div className="flex gap-3">
                  {isAdmin && (
                    <Button text="Start the game" color="primary" onClick={onStartGame} disabled={disableButton} />
                  )}
                  <Button text="Cancel" color="grey" onClick={onCancelGame} />
                </div>
              </div>
            </div>
          </div>
        </div>
      </Modal>
    </div>
  );
}
