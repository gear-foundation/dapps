import { useNavigate } from 'react-router-dom';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { clearZkData } from '@/features/zk/utils';
import { ROUTES } from '@/app/consts';
import { EVENT_NAME, SERVICE_NAME } from '../consts';
import { useState } from 'react';

type PlayerDeletedEvent = {
  game_id: string;
  removable_player: string;
};

export function useEventPlayerDeleted() {
  const { account } = useAccount();
  const program = useProgram();
  const navigate = useNavigate();
  const { game, triggerGame, resetGameState } = useMultiplayerGame();

  const [isPlayerDeleted, setIsPlayerDeleted] = useState(false);

  const onPlayerDeleted = async () => {
    if (!account) return;

    await triggerGame();
    clearZkData('multi', account);
    resetGameState();
    setIsPlayerDeleted(false);
    navigate(ROUTES.HOME);
  };

  const onData = async ({ game_id, removable_player }: PlayerDeletedEvent) => {
    if (!account || game?.admin !== game_id || removable_player !== account.decodedAddress) {
      return;
    }

    setIsPlayerDeleted(true);
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_PLAYER_DELETED_EVENT,
    onData,
  });

  return { onPlayerDeleted, isPlayerDeleted };
}
