import { useEffect, useRef } from 'react';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';

type PlacementVerifiedEvent = {
  admin: string;
};

export function useEventPlacementVerified() {
  const { game, triggerGame } = useMultiplayerGame();
  const program = useProgram();
  const event = useRef<Promise<() => void> | null>(null);

  const placementVerifiedEventCallback = ({ admin }: PlacementVerifiedEvent) => {
    if (admin !== game?.admin) {
      return;
    }

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
      event.current = program.multiple.subscribeToPlacementVerifiedEvent(placementVerifiedEventCallback);
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);
}
