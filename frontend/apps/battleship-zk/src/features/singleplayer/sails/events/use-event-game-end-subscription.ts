import { isNull } from '@polkadot/util';
import { BattleshipParticipants } from '@/app/utils/sails/lib/lib';
import { useProgram } from '@/app/utils/sails';
import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { gameEndResultAtom } from '../../atoms';
import { useSingleplayerGame } from '../../hooks';
import { useShips } from '@/features/zk/hooks/use-ships';
import { EVENT_NAME, SERVICE_NAME } from '../../consts';

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
        triggerGame();
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
