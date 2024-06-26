import { useEffect, useRef } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { program } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { useAccount } from '@gear-js/react-hooks';

type MoveVerifiedEvent = {
  step: number;
  result_: number;
  admin: string;
  opponent: string;
};

type StepResult = 0 | 1 | 2;

const config = {
  0: 'Boom',
  1: 'BoomShip',
  2: 'DeadShip',
};

export function useEventMoveVerifiedSubscription() {
  const event = useRef<Promise<() => void> | null>(null);
  const { account } = useAccount();
  const { game, triggerGame } = useMultiplayerGame();
  const { updateEnemyBoard } = useShips();
  const { clearProofData } = useProofShipHit();

  const moveVerifiedCallback = async (ev: MoveVerifiedEvent) => {
    const { opponent, admin } = ev;

    if (admin !== game?.admin || opponent !== account?.decodedAddress) {
      return;
    }

    const { step, result_ } = ev;

    updateEnemyBoard('multi', config[result_ as StepResult], step); //TODO update board

    triggerGame();
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
      event.current = program.multiple.subscribeToMoveVerifiedEvent(moveVerifiedCallback);
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);

  useEffect(() => {
    if (game === null) {
      clearProofData('multi');
    }
  }, [game]);
}
