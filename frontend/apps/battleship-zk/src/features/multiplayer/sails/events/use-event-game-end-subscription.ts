import { useEffect, useRef, useState } from 'react';
import { ParticipantInfo } from '@/features/game/assets/lib/lib';
import { program } from '@/app/utils/sails';

export type GameEndEvent = {
  winner: string;
  total_time: string | number;
  participants_info: [string, ParticipantInfo][];
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
      event.current = program.multiple.subscribeToEndGameEvent(gameEndCallback);
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
