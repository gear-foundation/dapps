import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { isNull } from '@polkadot/util';
import { useAtom } from 'jotai';

import { useProgram } from '@/app/utils/sails';
import { BattleshipParticipants } from '@/app/utils/sails/lib/lib';
import { useShips } from '@/features/zk/hooks/use-ships';

import { gameEndResultAtom } from '../../atoms';
import { EVENT_NAME, SERVICE_NAME } from '../../consts';
import { useSingleplayerGame } from '../../hooks';

export type GameEndEvent = {
  winner: BattleshipParticipants;
  time: string | number | bigint;
  total_shots: number;
  succesfull_shots: number;
  player: string;
  last_hit: number | null;
};

export function useEventGameEndSubscription() {
  const { account } = useAccount();
  const program = useProgram();
  const [gameEndResult, setGameEndResult] = useAtom(gameEndResultAtom);
  const { triggerGame } = useSingleplayerGame();
  const { updateEnemyBoard } = useShips();

  const onData = (ev: GameEndEvent) => {
    if (account?.decodedAddress !== ev.player) {
      return;
    }

    if (ev.winner) {
      setGameEndResult(ev);
      if (ev.winner === 'Player' && !isNull(ev.last_hit)) {
        updateEnemyBoard('single', 'DeadShip', ev.last_hit);
        void triggerGame();
      }
    }
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_END_GAME_EVENT,
    onData,
  });

  return { gameEndResult };
}
