import { useEffect, useRef, useState } from 'react';
import { BattleshipParticipants } from '@/app/utils/sails/lib/lib';
import { useProgram } from '@/app/utils/sails';
import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { gameEndResultAtom } from '../../atoms';

export type GameEndEvent = {
  winner: BattleshipParticipants;
  time: string | number | bigint;
  total_shots: number;
  succesfull_shots: number;
  player: string;
};

export function useEventGameEndSubscription() {
  const { account } = useAccount();
  const program = useProgram();
  const event = useRef<Promise<() => void> | null>(null);
  const [result, setResult] = useAtom(gameEndResultAtom);

  const gameEndCallback = (ev: GameEndEvent) => {
    if (account?.decodedAddress !== ev.player) {
      return;
    }

    if (ev.winner) {
      setResult(ev);
    }
  };

  const unsubscribeFromEvent = () => {
    if (event.current) {
      event.current?.then((unsubCallback) => {
        unsubCallback();
      });
    }
  };

  const subscribeToEvent = () => {
    if (!event.current) {
      event.current = program.single.subscribeToEndGameEvent((ev) => gameEndCallback(ev));
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);

  return { gameEndResult: result };
}
