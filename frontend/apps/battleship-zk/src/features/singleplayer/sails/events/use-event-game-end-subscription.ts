import { useEffect, useRef, useState } from 'react';
import { BattleshipParticipants } from '@/features/game/assets/lib/lib';
import { program } from '@/app/utils/sails';

export type GameEndEvent = {
  winner: BattleshipParticipants;
  time: string | number;
  total_shots: number;
  succesfull_shots: number;
};

export function useEventGameEndSubscription() {
  const event = useRef<Promise<() => void> | null>(null);
  const [result, setResult] = useState<GameEndEvent | null>(null);

  const gameEndCallback = (ev: GameEndEvent) => {
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
