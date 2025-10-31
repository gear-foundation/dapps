import { useAccount, useAlert, useProgramEvent } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { useProgram } from '@/app/utils/sails';
import { clearZkData } from '@/features/zk/utils';

import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { getIsPlacementStatus } from '../../utils';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type GameLeftEvent = {
  game_id: string;
};

export function useEventGameLeft() {
  const { account } = useAccount();
  const program = useProgram();
  const alert = useAlert();
  const navigate = useNavigate();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();

  const [isGameLeft, setIsGameLeft] = useState(false);

  const onGameLeft = async () => {
    if (!account) return;

    await triggerGame();
    navigate(ROUTES.HOME);
  };

  const onData = ({ game_id }: GameLeftEvent) => {
    if (!account || game?.admin !== game_id) {
      return;
    }

    if (game?.admin === account?.decodedAddress) {
      if (getIsPlacementStatus(game)) {
        setIsGameLeft(true);
      } else {
        alert.info('Your opponent has left the game.');
        void onGameLeft();
      }
    } else {
      void triggerGame().then(() => {
        clearZkData('multi', account);
        resetGameState();
        setIsGameLeft(false);
        navigate(ROUTES.HOME);
      });
    }
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_GAME_LEFT_EVENT,
    onData,
  });

  return { isGameLeft, onGameLeft };
}
