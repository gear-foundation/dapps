import { useAccount, useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEzTransactions } from 'gear-ez-transactions';
import { useAtom, useSetAtom } from 'jotai';
import { useEffect, useState } from 'react';

import { useApp } from '@/app/context/ctx-app';
import { useGame } from '@/app/context/ctx-game';
import { cn, useCancelTournamentMessage } from '@/app/utils';
import { SpriteIcon } from '@/components/ui/sprite-icon';
import { GAME_OVER, PRIZE_POOL } from '@/feature/game/consts';

import { ConfirmCancelModal } from '../modals/confirm-cancel';

type ParticipantSummary = {
  address: string;
  name: string;
  timeInMs: number;
  points: number;
};

export const GamePlayers = () => {
  const { api } = useApi();
  const { account } = useAccount();
  const { tournamentGame } = useGame();
  const { isPending, setIsPending } = useApp();
  const [prizePool, setPrizePool] = useAtom(PRIZE_POOL);

  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { gasless } = useEzTransactions();

  const setGameOver = useSetAtom(GAME_OVER);
  const [sortedParticipants, setSortedParticipants] = useState<ParticipantSummary[]>([]);
  const [isOpenCancelModal, setIsOpenCancelModal] = useState(false);

  const startTime = tournamentGame && 'started' in tournamentGame.stage ? Number(tournamentGame.stage.started) : 0;
  const durationMs = tournamentGame?.duration_ms || 0;
  const endTime = startTime + durationMs;

  const [timeRemaining, setTimeRemaining] = useState(Math.max(endTime - Date.now(), 0));

  const isAdmin = tournamentGame?.admin === account?.decodedAddress;

  const onCancelGame = () => {
    if (gasless.isLoading) {
      return;
    }

    setIsPending(true);
    void cancelTournamentMessage({
      onSuccess: () => {
        setIsPending(false);
        setTimeRemaining(0);
        setGameOver(false);
      },
      onError: () => {
        setIsPending(false);
        setTimeRemaining(0);
      },
    });
  };

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = parseFloat(String(tournamentGame?.bid).replace(/,/g, '') || '0') / 10 ** decimals;

  useEffect(() => {
    const updateTimer = () => {
      const now = Date.now();
      const remainingMs = endTime - now;

      if (remainingMs <= 0 && tournamentGame && 'started' in tournamentGame.stage) {
        setGameOver(true);
      } else if (remainingMs <= 0) {
        setTimeRemaining(0);
      } else {
        setTimeRemaining(Math.max(remainingMs, 0));
      }
    };

    const timerId = window.setInterval(updateTimer, 1000);

    return () => window.clearInterval(timerId);
  }, [endTime, setGameOver, setTimeRemaining, tournamentGame]);

  useEffect(() => {
    if (tournamentGame) {
      const pool = bid * tournamentGame?.participants.length;

      setPrizePool(pool);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!tournamentGame?.participants) {
      setSortedParticipants([]);
      return;
    }

    const updatedParticipants = tournamentGame.participants
      .map(([address, participant]) => ({
        address,
        name: participant.name,
        timeInMs: Number(participant.time),
        points: Number(participant.points),
      }))
      .sort((a, b) => {
        if (a.points !== b.points) {
          return b.points - a.points;
        }

        return a.timeInMs - b.timeInMs;
      });

    setSortedParticipants(updatedParticipants);
  }, [tournamentGame]);

  const minutesRemaining = Math.floor(timeRemaining / 60000);
  const secondsRemaining = Math.floor((timeRemaining % 60000) / 1000);
  const formattedTimeRemaining = `${minutesRemaining.toString().padStart(2, '0')}:${secondsRemaining
    .toString()
    .padStart(2, '0')}`;

  return (
    <div className="flex flex-col gap-4 items-center w-full">
      {isOpenCancelModal && (
        <ConfirmCancelModal setIsOpenCancelModal={setIsOpenCancelModal} onCancelGame={onCancelGame} />
      )}

      <h3 className="text-2xl font-bold">{tournamentGame?.tournament_name}</h3>
      <div className="flex gap-10 justify-between">
        <div className="flex gap-3">
          <p className="text-[#555756]">Prize pool:</p>
          <div className="flex gap-3 font-semibold">
            <SpriteIcon name="vara-coin" height={24} width={24} />
            {prizePool}
          </div>
        </div>

        <div className="flex gap-3">
          <p className="text-[#555756]">Tournament ends:</p>
          <div className="flex gap-3 font-semibold">{formattedTimeRemaining}</div>
        </div>
      </div>

      <div className="flex flex-col gap-3 w-full">
        {sortedParticipants.map((participant, index) => {
          const isActivePlayer = account?.decodedAddress === participant.address;
          const participantMinutes = Math.floor(participant.timeInMs / 60000);
          const participantSeconds = Math.floor((participant.timeInMs % 60000) / 1000);
          const timeFormatted = `${participantMinutes.toString().padStart(2, '0')}:${participantSeconds
            .toString()
            .padStart(2, '0')}`;

          return (
            <div
              key={participant.address}
              className={cn(
                'flex items-center justify-between p-2 bg-white border border-[#EDEDED] rounded-lg',
                isActivePlayer && 'bg-[#00FFC4] border-[#00EDB6]',
              )}>
              <div
                className={cn(
                  'py-2 px-3 rounded mr-3 flex justify-center gap-2 font-semibold',
                  isActivePlayer ? 'bg-[#00F5BC]' : 'bg-[#F5F5F5]',
                  index === 0 && 'bg-[#F9DC93]',
                  index > 2 && 'px-7',
                )}>
                {index + 1}
                {index === 0 && <SpriteIcon name="medal-fill" height={24} width={24} />}
                {(index === 1 || index === 2) && <SpriteIcon name="medal-line" height={24} width={24} />}
              </div>
              <div className="flex items-center gap-3">
                <p className="font-semibold w-10 lg:w-20 text-ellipsis overflow-hidden">{participant.name}</p>
              </div>
              <div className="flex items-center justify-end gap-1 lg:w-full lg:mr-20">
                <SpriteIcon name="game-time" height={16} width={16} />
                <p className="font-semibold">{timeFormatted}</p>
              </div>
              <div className="flex items-center gap-3">
                <SpriteIcon name="game-trophy" height={24} width={24} />
                <p className="font-semibold">{participant.points}</p>
              </div>
            </div>
          );
        })}
      </div>
      <div className="flex gap-3 justify-between w-full">
        {isAdmin && (
          <Button
            className="!bg-[#EB5757] !text-white !text-[14px] w-full"
            text="Cancel tournament"
            onClick={() => setIsOpenCancelModal(true)}
            isLoading={isPending}
          />
        )}
      </div>
    </div>
  );
};
