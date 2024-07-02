import { useEffect, useRef } from 'react';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';

type PlayerJoinedEvent = {
  player_id: string;
  game_id: string;
};

export function useEventPlayerJoinedGame() {
  const { game, triggerGame } = useMultiplayerGame();
  const program = useProgram();
  const event = useRef<Promise<() => void> | null>(null);

  const playerJoinedEventCallback = ({ game_id }: PlayerJoinedEvent) => {
    if (game?.admin === game_id) {
      triggerGame();
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
      event.current = program.multiple.subscribeToJoinedTheGameEvent(playerJoinedEventCallback);
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);
}
