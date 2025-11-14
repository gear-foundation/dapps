import { useAccount } from '@gear-js/react-hooks';
import { useEzTransactions } from 'gear-ez-transactions';
import { useAtom, useSetAtom } from 'jotai';

import { useApp } from '@/app/context/ctx-app';
import { useGame } from '@/app/context/ctx-game';
import { useCancelTournamentMessage, useLeaveGameMessage } from '@/app/utils';
import { Button } from '@/components';
import { Icons } from '@/components/ui/icons';
import { Modal } from '@/components/ui/modal/modal2';
import { SpriteIcon } from '@/components/ui/sprite-icon';
import { GAME_OVER, COINS, PRIZE_POOL } from '@/feature/game/consts';

type Props = {
  tournamentGame: TournamentState;
};

export const GameOverModal = ({ tournamentGame }: Props) => {
  const { gasless } = useEzTransactions();
  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { leaveGameMessage } = useLeaveGameMessage();

  const { account } = useAccount();
  const [, setGameOver] = useAtom(GAME_OVER);
  const setCoins = useSetAtom(COINS);
  const [prizePool] = useAtom(PRIZE_POOL);
  const { isPending, setIsPending } = useApp();
  const { setPreviousGame } = useGame();

  if (!('finished' in tournamentGame.stage)) {
    return null;
  }

  const isAdmin = tournamentGame.admin === account?.decodedAddress;

  const onResetGame = () => {
    setGameOver(false);
    setCoins({ gold: 0, silver: 0 });
  };

  const onError = () => {
    setIsPending(false);
  };

  const onSuccess = () => {
    setIsPending(false);
  };

  const onCancelGame = async () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      if (isAdmin) {
        await cancelTournamentMessage({ onError, onSuccess });
      } else {
        await leaveGameMessage({
          onError,
          onSuccess: () => {
            onSuccess();
            setPreviousGame(null);
          },
        });
      }
    }
  };

  const winners = tournamentGame.stage.finished.reduce(
    (acc, winnerAddress: string) => {
      const participant = tournamentGame.participants.find(([address]) => address === winnerAddress);

      return participant ? [...acc, participant] : acc;
    },
    [] as [string, Player][],
  );

  return (
    <div>
      <Modal open>
        <Modal.Content classNameContent="max-w-[600px]">
          <div className="flex flex-col justify-center gap-5 text-center">
            <h3 className="text-3xl font-semibold lg:text-center text-left">Game Over</h3>
            {winners.length > 1 ? (
              winners.map(([address, participant]) => (
                <div
                  key={address}
                  className="flex flex-col lg:flex-row items-center justify-between gap-3 w-4/5 mx-auto">
                  <div>
                    <p className="text-[#555756]">{participant.name}</p>
                  </div>
                  <div className="flex items-center gap-5 ml-5">
                    <div className="bg-[#F7F9FA] w-full font-medium flex gap-5 justify-center items-center">
                      <span className="flex items-center gap-1 font-semibold">
                        <Icons.statsCoins width={20} height={20} />
                        {Number(participant.points)}
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
                      {Number(winners?.[0][1].points)}
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
                  void onCancelGame();
                }}
                disabled={isPending}
                className="w-full">
                Close
              </Button>
              <Button onClick={() => void onCancelGame()} isLoading={isPending} disabled={isPending} className="w-full">
                Play again
              </Button>
            </div>
          </div>
        </Modal.Content>
      </Modal>
    </div>
  );
};
