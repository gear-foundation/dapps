import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { isNull } from '@polkadot/util';
import { useCallback } from 'react';

import { useProgram } from '@/app/utils/sails';
import { ParticipantInfo } from '@/app/utils/sails/lib/lib';
import { useShips } from '@/features/zk/hooks/use-ships';

import { useMultiplayerGame } from '../../hooks';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

export type GameEndEvent = {
  winner: string;
  total_time: string | number | bigint;
  participants_info: [string, ParticipantInfo][];
  admin: string;
  last_hit: number | null;
};

export function useEventGameEndSubscription() {
  const { account } = useAccount();
  const program = useProgram();
  const { game, gameEndResult, setGameEndResult, triggerGame } = useMultiplayerGame();
  const { updateEnemyBoard } = useShips();

  const onData = useCallback(
    (ev: GameEndEvent) => {
      if (!account?.decodedAddress) {
        return;
      }

      const { participants_info } = ev;

      const isParticipant = participants_info.map((item) => item[0]).includes(account.decodedAddress);

      if (!isParticipant) {
        return;
      }

      if (ev.winner) {
        setGameEndResult(ev);

        if (ev.winner === account?.decodedAddress && !isNull(ev.last_hit)) {
          updateEnemyBoard('multi', 'DeadShip', ev.last_hit);
          void triggerGame();
        }
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [account?.decodedAddress, game?.admin],
  );

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_END_GAME_EVENT,
    onData,
  });

  return { gameEndResult };
}
