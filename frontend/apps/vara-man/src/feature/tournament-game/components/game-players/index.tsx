import React, { useEffect, useState } from 'react';
import { useAtom, useSetAtom } from 'jotai';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';

import { useGame } from '@/app/context/ctx-game';
import { cn, useCancelTournamentMessage } from '@/app/utils';
import { SpriteIcon } from '@/components/ui/sprite-icon';
import { useApp } from '@/app/context/ctx-app';
import { GAME_OVER, PRIZE_POOL } from '@/feature/game/consts';
import { ConfirmCancelModal } from '../modals/confirm-cancel';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';

export const GamePlayers = () => {
  const { api } = useApi();
  const { account } = useAccount();
  const { tournamentGame } = useGame();
  const { isPending, setIsPending } = useApp();
  const [prizePool, setPrizePool] = useAtom(PRIZE_POOL);

  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { gasless } = useEzTransactions();

  const setGameOver = useSetAtom(GAME_OVER);
  const [sortedParticipants, setSortedParticipants] = useState<any>([]);
  const [isOpenCancelModal, setIsOpenCancelModal] = useState(false);

  const startTime = (tournamentGame && 'started' in tournamentGame.stage && Number(tournamentGame.stage.started)) || 0;
  const durationMs = tournamentGame?.duration_ms || 0;
  const endTime = startTime + durationMs;

  const [timeLeft, setTimeLeft] = useState(Math.max(endTime - Date.now(), 0));

  const isAdmin = tournamentGame?.admin === account?.decodedAddress;

  const onCancelGame = () => {
    if (!gasless.isLoading) {
      setIsPending(true);
      cancelTournamentMessage({
        onSuccess: () => {
          setIsPending(false);
          setTimeLeft(0);
          setGameOver(false);
        },
        onError: () => {
          setIsPending(false);
          setTimeLeft(0);
        },
      });
    }
  };

  const [decimals] = api?.registry.chainDecimals ?? [12];
  const bid = parseFloat(String(tournamentGame?.bid).replace(/,/g, '') || '0') / 10 ** decimals;

  useEffect(() => {
    const updateTimer = () => {
      const now = Date.now();
      const timeLeft = endTime - now;
      if (timeLeft <= 0 && tournamentGame && 'started' in tournamentGame.stage) {
        setGameOver(true);
      } else if (timeLeft <= 0) {
        setTimeLeft(0);
      } else {
        setTimeLeft(Math.max(timeLeft, 0));
      }
    };

    const timerId = setInterval(updateTimer, 1000);

    return () => clearInterval(timerId);
  }, [endTime, tournamentGame]);

  const minutes = Math.floor(timeLeft / 60000);
  const seconds = Math.floor((timeLeft % 60000) / 1000);

  const formattedTimeLeft = `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;

  useEffect(() => {
    if (tournamentGame) {
      const pool = bid * tournamentGame?.participants.length;

      setPrizePool(pool);
    }
  }, []);

  useEffect(() => {
    if (!tournamentGame?.participants) {
      return;
    }

    const sortedParticipants = tournamentGame.participants
      .map((participant) => {
        const timeInMs = Number(participant[1].time);
        const points = Number(participant[1].points);
        return {
          address: participant[0],
          name: participant[1].name,
          timeInMs,
          points,
        };
      })
      .sort((a, b) => {
        if (a.points !== b.points) return b.points - a.points;
        return a.timeInMs - b.timeInMs;
      });

    setSortedParticipants(sortedParticipants);
  }, [tournamentGame]);

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
          <div className="flex gap-3 font-semibold">{formattedTimeLeft}</div>
        </div>
      </div>

      <div className="flex flex-col gap-3 w-full">
        {sortedParticipants?.map(
          (
            participant: {
              address: React.Key | null | undefined;
              timeInMs: number;
              name: string;
              points: number;
            },
            index: number,
          ) => {
            const isActivePlayer = account?.decodedAddress === participant.address;
            const minutes = Math.floor(participant.timeInMs / 60000);
            const seconds = Math.floor((participant.timeInMs % 60000) / 1000);
            const timeFormatted = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;

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
          },
        )}
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
