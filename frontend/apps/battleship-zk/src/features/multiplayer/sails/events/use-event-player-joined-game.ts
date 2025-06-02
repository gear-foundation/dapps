import { useAccount, useAlert, useProgramEvent } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils/sails';

import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type PlayerJoinedEvent = {
  player_id: string;
  game_id: string;
};

export function useEventPlayerJoinedGame() {
  const { game, triggerGame } = useMultiplayerGame();
  const program = useProgram();
  const alert = useAlert();
  const { account } = useAccount();

  const onData = ({ game_id, player_id }: PlayerJoinedEvent) => {
    if (game?.admin === game_id) {
      triggerGame();

      if (player_id !== account?.decodedAddress) {
        alert.info('The player has joined.');
      }
    }
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_JOINED_THE_GAME_EVENT,
    onData,
  });
}
