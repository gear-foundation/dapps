import { useNavigate } from 'react-router-dom';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useAccount, useAlert, useProgramEvent } from '@gear-js/react-hooks';
import { clearZkData } from '@/features/zk/utils';
import { ROUTES } from '@/app/consts';
import { EVENT_NAME, SERVICE_NAME } from '../consts';
import { useState } from 'react';
import { getIsPlacementStatus } from '../../utils';

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

  const onData = async ({ game_id }: GameLeftEvent) => {
    console.log('! GameLeft myacc:', account?.decodedAddress, game?.admin);

    if (!account || game?.admin !== game_id) {
      return;
    }

    if (game?.admin === account?.decodedAddress) {
      console.log(2);
      if (getIsPlacementStatus(game)) {
        setIsGameLeft(true);
      } else {
        alert.info('Your opponent has left the game.');
        onGameLeft();
      }
    } else {
      await triggerGame();
      clearZkData('multi', account);
      resetGameState();
      setIsGameLeft(false);
      navigate(ROUTES.HOME);
      console.log(1);
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
