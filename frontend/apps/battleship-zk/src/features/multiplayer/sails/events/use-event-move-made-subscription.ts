import { useAccount } from '@gear-js/react-hooks';
import { isNull } from '@polkadot/util';
import { useEffect, useRef } from 'react';

import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { MultipleUtilsStepResult } from '@/app/utils/sails/lib/lib';
import { stepResultToBoardEntityMap } from '@/features/game/consts';

type MoveMadeEvent = {
  game_id: string;
  step: number | null;
  verified_result: [number, MultipleUtilsStepResult] | null;
  turn: string;
};

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const gameType = 'multi';
  const event = useRef<Promise<() => void> | null>(null);
  const { account } = useAccount();
  const { game, triggerGame } = useMultiplayerGame();
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

  const moveMadeCallback = async (ev: MoveMadeEvent) => {
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
  };

  const unsubscribeFromEvent = () => {
    if (event.current) {
      event.current?.then((unsubCallback) => {
        unsubCallback();
      });
    }
  };

  const subscribeToEvent = () => {
    if (!event.current) {
      event.current = program.multiple.subscribeToMoveMadeEvent(moveMadeCallback);
    }
  };

  useEffect(() => {
    subscribeToEvent();

    return () => {
      unsubscribeFromEvent();
    };
  }, []);

  useEffect(() => {
    if (game === null) {
      clearProofData(gameType);
    }
  }, [game]);
}
