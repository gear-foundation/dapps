import { useEffect, useRef } from 'react';
import { program } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';

export function useEventPlacementVerified() {
  const { triggerGame } = useMultiplayerGame();
  const event = useRef<Promise<() => void> | null>(null);

  const placementVerifiedEventCallback = () => {
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
