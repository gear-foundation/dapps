import { useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { clearZkData } from '@/features/zk/utils';
import { ROUTES } from '@/app/consts';

type GameCancelledEvent = {
  game_id: string;
};

export function useEventGameCancelled() {
  const { account } = useAccount();
  const program = useProgram();
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
    navigate(ROUTES.HOME);
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
