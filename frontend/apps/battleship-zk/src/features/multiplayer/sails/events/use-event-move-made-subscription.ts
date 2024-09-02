import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { isNull } from '@polkadot/util';
import { useEffect } from 'react';

import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { MultipleUtilsStepResult } from '@/app/utils/sails/lib/lib';
import { stepResultToBoardEntityMap } from '@/features/game/consts';
import { usePending } from '@/features/game/hooks';
import { EVENT_NAME, SERVICE_NAME } from '../consts';

type MoveMadeEvent = {
  game_id: string;
  step: number | null;
  verified_result: [number, MultipleUtilsStepResult] | null;
  turn: string;
};

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const gameType = 'multi';
  const { account } = useAccount();
  const { game, triggerGame } = useMultiplayerGame();
  const { setPending } = usePending();
  const { getPlayerShips, getPlayerHits, updatePlayerHits, updatePlayerBoard, updateEnemyBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const generateProofHit = async (ev: MoveMadeEvent) => {
    const ships = getPlayerShips(gameType);
    const hits = getPlayerHits(gameType);

    if (!ships || !hits || isNull(ev.step)) {
      return;
    }

    const proofData = await requestProofHit(
      ships,
      ev.step.toString(),
      hits.map((item) => item.toString()),
    );

    return proofData;
  };

  const onData = async (ev: MoveMadeEvent) => {
    const { game_id, turn, step, verified_result } = ev;

    if (game_id !== game?.admin || turn !== account?.decodedAddress) {
      return;
    }

    const proofData = await generateProofHit(ev);

    if (!isNull(step)) {
      updatePlayerBoard(gameType, step);
      updatePlayerHits(gameType, step);
    }

    if (!isNull(verified_result)) {
      const [lastHit, stepResult] = verified_result;

      updateEnemyBoard(gameType, stepResultToBoardEntityMap[stepResult], lastHit);
    }

    saveProofData(gameType, proofData);

    await triggerGame();
    setPending(false);
  };

  useProgramEvent({
    program,
    serviceName: SERVICE_NAME,
    functionName: EVENT_NAME.SUBSCRIBE_TO_MOVE_MADE_EVENT,
    onData,
  });

  useEffect(() => {
    if (game === null) {
      clearProofData(gameType);
    }
  }, [game]);
}
