import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { useProgram } from '@/app/utils/sails';
import { clearZkData } from '@/features/zk/utils';

import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type GameCancelledEvent = {
  game_id: string;
};

export function useEventGameCancelled() {
  const { account } = useAccount();
  const program = useProgram();
  const navigate = useNavigate();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();

  const [isGameCancelled, setIsGameCancelled] = useState(false);

  const onGameCancelled = async () => {
    if (!account) return;

    await triggerGame();
    clearZkData('multi', account);
    resetGameState();
    setIsGameCancelled(false);
    navigate(ROUTES.HOME);
  };

  const onData = ({ game_id }: GameCancelledEvent) => {
    if (!account || game?.admin !== game_id) {
      return;
    }

    if (game?.admin === account?.decodedAddress) {
      void onGameCancelled();
    } else {
      setIsGameCancelled(true);
    }
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_GAME_CANCELED_EVENT,
    onData,
  });

  return { isGameCancelled, onGameCancelled };
}
