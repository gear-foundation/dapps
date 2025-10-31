import { useProgramEvent } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils/sails';

import { useMultiplayerGame } from '../../hooks/use-multiplayer-game';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type PlacementVerifiedEvent = {
  admin: string;
};

export function useEventPlacementVerified() {
  const { game, triggerGame } = useMultiplayerGame();
  const program = useProgram();

  const onData = ({ admin }: PlacementVerifiedEvent) => {
    if (admin !== game?.admin) {
      return;
    }

    void triggerGame();
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_PLACEMENT_VERIFIED_EVENT,
    onData,
  });
}
