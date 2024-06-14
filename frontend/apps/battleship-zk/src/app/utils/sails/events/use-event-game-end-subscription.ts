import { useEffect, useState } from 'react';
import { sails } from '../sails';

type BattleshipParticipants = 'Player' | 'Bot';

type GameEndEvent = {
  winner: BattleshipParticipants;
  time: number;
  total_shots: number;
  succesfull_shots: number;
};

export function useEventGameEndSubscription() {
  const [result, setResult] = useState<GameEndEvent | null>(null);

  const gameEndCallback = (ev: GameEndEvent) => {
    if (ev.winner) {
      setResult(ev);
    }
  };

  useEffect(() => {
    sails.services.Single.events.EndGame.subscribe((ev: GameEndEvent) => gameEndCallback(ev));
  }, []);

  return { result };
}
