import { useAccount } from '@gear-js/react-hooks';
import { atom, useAtom, useSetAtom } from 'jotai';
import { useEffect, useRef, useState } from 'react';

import { BattleState, Pair, Player } from '@/app/utils';
import { useEventRoundActionSubscription } from '@/app/utils/sails/events';

import { MAX_HEALTH, TIME_LEFT_GAP } from './consts';
import {
  battleHistoryAtom,
  battleHistoryStorage,
  characterStatsStorage,
  currentPlayersAtom,
  currentPlayersStorage,
  isBattleCanceledAtom,
  otherPairBattleWatchAtom,
  warriorIdStorage,
} from './store';
import { BattleHistory } from './types';

const pendingAtom = atom<boolean>(false);

export function usePending() {
  const [pending, setPending] = useAtom(pendingAtom);

  return { pending, setPending };
}

export function useResetGameState() {
  const setBattleHistory = useSetAtom(battleHistoryAtom);
  const setOtherPairBattleWatch = useSetAtom(otherPairBattleWatchAtom);
  const setIsBattleCanceledAtom = useSetAtom(isBattleCanceledAtom);
  useEffect(() => {
    setBattleHistory(null);
    battleHistoryStorage.set(null);
    setOtherPairBattleWatch(null);
    setIsBattleCanceledAtom(false);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
}

export function useResetCharacterStats() {
  useEffect(() => {
    characterStatsStorage.set(null);
    warriorIdStorage.set(null);
  }, []);
}

export type UsePrepareBattleHistoryParams = {
  pair: Pair;
  player: Player;
  opponent: Player | null;
  turnEndCallback: () => void;
};

export function usePrepareBattleHistory({ pair, player, opponent, turnEndCallback }: UsePrepareBattleHistoryParams) {
  const setBattleHistory = useSetAtom(battleHistoryAtom);
  const { lastMoves, resetLastMoves } = useEventRoundActionSubscription(pair);
  const setCurrentPlayers = useSetAtom(currentPlayersAtom);

  const myFullDefence = player?.player_settings.defence === 100;
  const opponentFullDefence = opponent?.player_settings.defence === 100;

  useEffect(() => {
    if (lastMoves && opponent) {
      const [myMove, opponentsMove] = lastMoves.moves;
      const [myHealth, opponentsHealth] = lastMoves.newHealth;

      setBattleHistory((prev) => {
        const myReceivedDamage = (prev?.[0].player.health ?? MAX_HEALTH) - myHealth;
        const opponentsReceivedDamage = (prev?.[0].opponent.health ?? MAX_HEALTH) - opponentsHealth;
        const isBothUseReflect = myMove === 'Reflect' && opponentsMove === 'Reflect';
        const meReflectAll = myMove === 'Reflect' && myFullDefence;
        const opponentReflectAll = opponentsMove === 'Reflect' && opponentFullDefence;

        const newHistory: BattleHistory = {
          player: {
            action: myMove,
            isDodged: myReceivedDamage === 0 && !isBothUseReflect && !meReflectAll,
            receivedDamage: myReceivedDamage,
            health: myHealth,
          },
          opponent: {
            action: opponentsMove,
            isDodged: opponentsReceivedDamage === 0 && !isBothUseReflect && !opponentReflectAll,
            receivedDamage: opponentsReceivedDamage,
            health: opponentsHealth,
          },
        };

        const next = prev ? [newHistory, ...prev] : [newHistory];
        battleHistoryStorage.set(next);
        return next;
      });

      setCurrentPlayers((prev) => {
        if (prev) {
          const newPlayers = { ...prev };
          newPlayers.player.player_settings.health = myHealth;
          newPlayers.opponent.player_settings.health = opponentsHealth;
          currentPlayersStorage.set(newPlayers);
          return newPlayers;
        }
        return prev;
      });

      turnEndCallback();
      resetLastMoves();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [lastMoves, myFullDefence, opponentFullDefence]);
}

export type UseTimerParams = {
  remainingTime: string | number | bigint | null | undefined;
  shouldGoOn?: boolean;
};

export function useTimer({ remainingTime, shouldGoOn = true }: UseTimerParams) {
  const [timeLeft, setTimeLeft] = useState<number | null>(null);
  const startTimeRef = useRef<number | null>(null);

  useEffect(() => {
    if (remainingTime === undefined) {
      setTimeLeft(null);
      startTimeRef.current = null;
    } else if (remainingTime === 0) {
      setTimeLeft(0);
    } else {
      const updateTimer = () => {
        if (!shouldGoOn) {
          return;
        }
        const currentTime = new Date().getTime();
        if (startTimeRef.current === null) {
          startTimeRef.current = currentTime;
        }
        const timeLeftMilliseconds =
          Number(remainingTime) + (startTimeRef.current || currentTime) - currentTime - TIME_LEFT_GAP;

        setTimeLeft(Math.max(timeLeftMilliseconds, 0));
      };

      const timerInterval = setInterval(updateTimer, 1000);

      return () => {
        clearInterval(timerInterval);
      };
    }
  }, [shouldGoOn, remainingTime]);

  const displayedTime = timeLeft ?? (remainingTime ? Math.max(Number(remainingTime), 0) : null);
  const formattedTimeLeft = displayedTime !== null ? Math.round(displayedTime / 1000) : '';

  return formattedTimeLeft;
}

export function useParticipants(battleState?: BattleState | null) {
  const { account } = useAccount();
  const [otherPairBattleWatch] = useAtom(otherPairBattleWatchAtom);
  const [currentPlayers, setCurrentPlayers] = useAtom(currentPlayersAtom);

  const { pairs, players_to_pairs } = battleState || {};
  const pairId =
    otherPairBattleWatch ?? players_to_pairs?.find(([address]) => account?.decodedAddress === address)?.[1];
  const pair = pairs?.find(([number]) => pairId === number)?.[1];

  const { participants, defeated_participants } = battleState || {};

  const { player_1, player_2 } = pair || {};
  const isAlive = Boolean(participants?.some(([address]) => address === account?.decodedAddress));

  const allParticipants = participants && defeated_participants ? [...participants, ...defeated_participants] : [];

  const participantsMap = allParticipants.reduce(
    (acc, [key, player]) => {
      acc[key] = player;
      return acc;
    },
    {} as Record<string, Player>,
  );

  const playerAddress = otherPairBattleWatch !== null ? player_1 : account?.decodedAddress;
  const opponentsAddress = playerAddress === player_1 ? player_2 : player_1;

  const player = playerAddress ? participantsMap[playerAddress] : null;
  const opponent = opponentsAddress ? participantsMap[opponentsAddress] : null;

  const hasPlayer = Boolean(player);
  const hasOpponent = Boolean(opponent);

  useEffect(() => {
    if (player && opponent) {
      setCurrentPlayers({ player, opponent });
      currentPlayersStorage.set({ player, opponent });
    }
  }, [player, opponent, setCurrentPlayers]);

  return { participantsMap, allParticipants, hasPlayer, hasOpponent, isAlive, pair, currentPlayers };
}
