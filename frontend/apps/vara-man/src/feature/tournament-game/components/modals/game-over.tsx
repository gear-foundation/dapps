import { Key } from 'react';
import { useAtom, useSetAtom } from 'jotai';
import { useAccount } from '@gear-js/react-hooks';

import { Button } from '@/components';
import { Icons } from '@/components/ui/icons';
import { useGame } from '@/app/context/ctx-game';
import { useApp } from '@/app/context/ctx-app';
import { ITournamentGameInstance } from '@/app/types/game';
import { GAME_OVER, COINS, PRIZE_POOL } from '@/feature/game/consts';
import { useGameMessage } from '@/app/hooks/use-game';

import { SpriteIcon } from '@/components/ui/sprite-icon';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { Modal } from '@/components/ui/modal/modal2';

type Props = {
  tournamentGame: ITournamentGameInstance;
};

export const GameOverModal = ({ tournamentGame }: Props) => {
  const handleMessage = useGameMessage();
  const { gasless, signless } = useEzTransactions();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const gasLimit = 120000000000;

  const { account } = useAccount();
  const [, setGameOver] = useAtom(GAME_OVER);
  const setCoins = useSetAtom(COINS);
  const [prizePool] = useAtom(PRIZE_POOL);
  const { isPending, setIsPending } = useApp();
  const { setPreviousGame } = useGame();

  const isAdmin = tournamentGame[0].admin === account?.decodedAddress;

  const onResetGame = () => {
    setGameOver(false);
    setCoins({ gold: 0, silver: 0 });
  };

  const onSuccess = () => {
    setIsPending(false);
  };
  const onError = () => {
    setIsPending(false);
  };

  const onCancelGame = () => {
    if (!gasless.isLoading) {
      if (isAdmin) {
        checkBalance(gasLimit, () => {
          setIsPending(true);
          handleMessage({
            payload: {
              CancelTournament: {},
            },
            voucherId: gasless.voucherId,
            gasLimit,
            onSuccess,
            onError,
          });
        });
      } else {
        checkBalance(gasLimit, () => {
          setIsPending(true);
          handleMessage({
            payload: {
              LeaveGame: {},
            },
            voucherId: gasless.voucherId,
            gasLimit,
            onSuccess,
            onError,
          });
        });
        setPreviousGame(null);
      }
    }
  };

  const winners = tournamentGame[0].stage.Finished.map((winnerAddress: string) =>
    tournamentGame[0].participants.find(([address]) => address === winnerAddress),
  ).filter((winnerInfo: undefined) => winnerInfo !== undefined);

  return (
    <div>
      <Modal open>
        <Modal.Content classNameContent="max-w-[600px]">
          <div className="flex flex-col justify-center gap-5 text-center">
            <h3 className="text-3xl font-semibold lg:text-center text-left">Game Over</h3>
            {winners.length > 1 ? (
              winners.map((winner: { points: string; name: string }[], index: Key | null | undefined) => (
                <div key={index} className="flex flex-col lg:flex-row items-center justify-between gap-3 w-4/5 mx-auto">
                  <div>
                    <p className="text-[#555756]">{winner?.[1].name}</p>
                  </div>
                  <div className="flex items-center gap-5 ml-5">
                    <div className="bg-[#F7F9FA] w-fullfont-medium flex gap-5 justify-center items-center">
                      <span className="flex items-center gap-1 font-semibold">
                        <Icons.statsCoins width={20} height={20} />
                        {winner?.[1].points}
                      </span>
                    </div>
                    <div className="bg-[#F7F9FA] w-full font-medium flex gap-5 justify-center items-center">
                      <span className="flex items-center gap-1 font-semibold">
                        <SpriteIcon name="vara-coin" height={20} width={20} />
                        {prizePool} VARA
                      </span>
                    </div>
                  </div>
                </div>
              ))
            ) : (
              <div>
                <div>
                  <p className="text-[#555756] lg:mt-2 lg:mb-0 mb-4 text-left lg:text-center">
                    {winners?.[0][1].name} wins!
                  </p>
                </div>
                <div className="flex flex-col lg:flex-row">
                  <div className="bg-[#F7F9FA] w-full p-5 font-medium flex gap-5 justify-between lg:justify-center items-center">
                    Score:
                    <span className="flex items-center gap-2 font-semibold">
                      <Icons.statsCoins />
                      {winners?.[0][1].points}
                    </span>
                  </div>
                  <div className="bg-[#F7F9FA] w-full p-5 font-medium flex gap-5 justify-between lg:justify-center items-center">
                    Winner prize:
                    <span className="flex items-center gap-2 font-semibold">
                      <SpriteIcon name="vara-coin" height={24} width={24} />
                      {prizePool} VARA
                    </span>
                  </div>
                </div>
              </div>
            )}

            <div className="flex justify-evenly gap-5">
              <Button
                variant="gray"
                onClick={() => {
                  onResetGame();
                  onCancelGame();
                }}
                disabled={isPending}
                className="w-full">
                Close
              </Button>
              <Button onClick={onCancelGame} isLoading={isPending} disabled={isPending} className="w-full">
                Play again
              </Button>
            </div>
          </div>
        </Modal.Content>
      </Modal>
    </div>
  );
};
