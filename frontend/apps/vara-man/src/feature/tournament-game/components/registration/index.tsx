import { useGame } from '@/app/context/ctx-game';
import {
  TournamentState,
  cn,
  copyToClipboard,
  prettifyText,
  useCancelTournamentMessage,
  useStartTournamentMessage,
} from '@/app/utils';
import { SpriteIcon } from '@/components/ui/sprite-icon';
import { useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useApp } from '@/app/context/ctx-app';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useDeletePlayerMessage } from '@/app/utils/sails/messages/use-delete-player-message';
import { useCancelRegisterMessage } from '@/app/utils/sails/messages/use-cancel-register-message';

type Props = {
  tournamentGame: TournamentState;
  setPlayGame?: (value: boolean) => void;
};

export const Registration = ({ tournamentGame, setPlayGame }: Props) => {
  const alert = useAlert();
  const { api } = useApi();
  const { account } = useAccount();
  const { setPreviousGame, setTournamentGame } = useGame();
  const { isPending, setIsPending } = useApp();
  const { startTournamentMessage } = useStartTournamentMessage();
  const { deletePlayerMessage } = useDeletePlayerMessage();
  const { cancelRegisterMessage } = useCancelRegisterMessage();
  const { cancelTournamentMessage } = useCancelTournamentMessage();

  const { gasless } = useEzTransactions();
  const onError = () => {
    setIsPending(false);
  };
  const onSuccess = () => {
    setIsPending(false);
  };

  const isAdmin = tournamentGame?.admin === account?.decodedAddress;

  const onRemovePlayer = async (player: string) => {
    if (!gasless.isLoading) {
      setIsPending(true);
      await deletePlayerMessage(player, { onError, onSuccess });
    }
  };

  const onStartGame = async () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      await startTournamentMessage({
        onError,
        onSuccess: () => {
          setIsPending(false);
          setPlayGame && setPlayGame(true);
          onSuccess();
        },
      });
    }
  };

  const onCancelGame = async () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      if (isAdmin) {
        await cancelTournamentMessage({ onError, onSuccess });
      } else {
        await cancelRegisterMessage({
          onError,
          onSuccess: () => {
            setPreviousGame(null);
            setTournamentGame(undefined);
            onSuccess();
          },
        });
      }
    }
  };

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = parseFloat(String(tournamentGame?.bid).replace(/,/g, '') || '0') / 10 ** decimals;

  return (
    <div className="flex flex-col gap-4 items-center w-full">
      <h3 className="text-2xl font-bold">Registration</h3>
      <p className="text-[#555756]">
        Players ({tournamentGame?.participants.length}/10). Waiting for other players...{' '}
      </p>
      {isAdmin && (
        <div className="flex gap-2 font-medium">
          Share the game's address:
          <span className="font-bold">({prettifyText(account.address)})</span>
          <span
            className="font-semibold text-[#0ED3A3] cursor-pointer"
            onClick={() => copyToClipboard({ key: account.address, alert })}>
            Copy
          </span>
        </div>
      )}
      <div className="flex flex-col gap-3 w-full">
        {tournamentGame?.participants.map((player, index) => {
          const isActivePlayer = account?.decodedAddress === player[0];
          const { name, points, time } = player[1];

          return (
            <div
              key={player[0]}
              className={cn(
                'flex items-center justify-between p-2 bg-white border border-[#EDEDED] rounded-lg',
                isActivePlayer && 'bg-[#00FFC4] border-[#00EDB6]',
              )}>
              <div className="flex items-center gap-3">
                {isAdmin && !isActivePlayer && (
                  <button onClick={() => onRemovePlayer(player[0])}>
                    <SpriteIcon name="close-gray" height={24} width={24} />
                  </button>
                )}

                {isAdmin && isActivePlayer && <div className="py-2 px-3"></div>}

                {!isAdmin && <div className="bg-[#F5F5F5] font-semibold py-2 px-5 rounded">{index + 1}</div>}

                <p className="font-semibold">{name}</p>
              </div>
              <div className="flex items-center gap-3">
                <SpriteIcon name="vara-coin" height={24} width={24} />
                <p className="font-semibold">{bid}</p>
              </div>
            </div>
          );
        })}
      </div>

      <div className="flex gap-3 justify-between w-full">
        {isAdmin ? (
          <>
            <Button
              className="!bg-[#EB5757] !text-white md:!text-[14px] !px-3"
              text="Cancel tournament"
              onClick={onCancelGame}
              isLoading={isPending}
            />
            <Button
              className="md:!text-[14px] !px-3"
              text="Start tournament"
              onClick={onStartGame}
              isLoading={isPending}
            />
          </>
        ) : (
          <Button
            className="md:!text-[14px] w-full"
            color="grey"
            text="Cancel"
            onClick={onCancelGame}
            isLoading={isPending}
          />
        )}
      </div>
    </div>
  );
};
