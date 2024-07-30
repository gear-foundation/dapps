import { useEffect, useRef } from 'react';
import { isNull } from '@polkadot/util';
import { BattleshipParticipants } from '@/app/utils/sails/lib/lib';
import { useProgram } from '@/app/utils/sails';
import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { gameEndResultAtom } from '../../atoms';
import { useSingleplayerGame } from '../../hooks';
import { useShips } from '@/features/zk/hooks/use-ships';

export type GameEndEvent = {
  winner: BattleshipParticipants;
  time: string | number | bigint;
  total_shots: number;
  succesfull_shots: number;
  player: string;
  last_hit: number | null;
};

export function useEventGameEndSubscription() {
  const { account } = useAccount();
  const program = useProgram();
  const event = useRef<Promise<() => void> | null>(null);
  const [gameEndResult, setGameEndResult] = useAtom(gameEndResultAtom);
  const { triggerGame } = useSingleplayerGame();
  const { updateEnemyBoard } = useShips();

  const gameEndCallback = (ev: GameEndEvent) => {
    if (account?.decodedAddress !== ev.player) {
      return;
    }

    if (ev.winner) {
      setGameEndResult(ev);
      if (ev.winner === 'Player' && !isNull(ev.last_hit)) {
        updateEnemyBoard('single', 'DeadShip', ev.last_hit);
        triggerGame();
      }
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

  return { gameEndResult };
}
