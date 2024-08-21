import { useNavigate } from 'react-router-dom';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useAccount, useAlert, useProgramEvent } from '@gear-js/react-hooks';
import { clearZkData } from '@/features/zk/utils';
import { ROUTES } from '@/app/consts';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type GameCancelledEvent = {
  game_id: string;
};

export function useEventGameCancelled() {
  const { account } = useAccount();
  const program = useProgram();
  const alert = useAlert();
  const navigate = useNavigate();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();

  const onData = async ({ game_id }: GameCancelledEvent) => {
    if (!account || game?.admin !== game_id) {
      return;
    }

    await triggerGame();
    clearZkData('multi', account);
    resetGameState();
    navigate(ROUTES.HOME);
    alert.info('Admin has removed the game');
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_GAME_CANCELED_EVENT,
    onData,
  });
}
