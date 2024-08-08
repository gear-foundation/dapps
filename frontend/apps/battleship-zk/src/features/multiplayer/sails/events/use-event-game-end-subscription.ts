import { useCallback, useEffect, useState } from 'react';
import { isNull } from '@polkadot/util';
import { useShips } from '@/features/zk/hooks/use-ships';
import { ParticipantInfo } from '@/app/utils/sails/lib/lib';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
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
  const [gameAdmin, setGameAdmin] = useState<string | null>(null);
  const { updateEnemyBoard } = useShips();

  const onData = useCallback(
    async (ev: GameEndEvent) => {
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
          triggerGame();
        }
      }
    },
    [gameAdmin],
  );

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_END_GAME_EVENT,
    onData,
  });

  useEffect(() => {
    if (game?.admin) {
      setGameAdmin(game.admin);
    }
  }, [game?.admin]);

  return { gameEndResult };
}
