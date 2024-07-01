import { useEffect, useRef } from 'react';
import { program } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useNavigate } from 'react-router-dom';
import { clearZkData } from '@/features/zk/utils';

type GameCancelledEvent = {
  game_id: string;
};

export function useEventGameCancelled() {
  const { account } = useAccount();
  const alert = useAlert();
  const navigate = useNavigate();
  const { game, triggerGame } = useMultiplayerGame();
  const event = useRef<Promise<() => void> | null>(null);

  const gameCancelledEventCallback = async ({ game_id }: GameCancelledEvent) => {
    if (!account || game?.admin !== game_id) {
      return;
    }

    await triggerGame();
    clearZkData('multi', account);
    navigate('/');
    alert.info('Admin has removed the game');
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
      event.current = program.multiple.subscribeToGameCanceledEvent(gameCancelledEventCallback);
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);
}
