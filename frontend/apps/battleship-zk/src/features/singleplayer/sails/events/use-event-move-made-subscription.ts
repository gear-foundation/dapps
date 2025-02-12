import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { isNull } from '@polkadot/util';
import { useEffect } from 'react';

import { useProgram } from '@/app/utils/sails';
import { SingleUtilsStepResult } from '@/app/utils/sails/lib/lib';
import { stepResultToBoardEntityMap } from '@/features/game/consts';
import { usePending } from '@/features/game/hooks';
import { useSingleplayerGame } from '@/features/singleplayer/hooks/use-singleplayer-game';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';

import { EVENT_NAME, SERVICE_NAME } from '../../consts';

type MoveMadeEvent = {
  bot_step: number | null;
  step: number | null;
  step_result: SingleUtilsStepResult | null;
  player: string;
};

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const gameType = 'single';
  const { account } = useAccount();
  const { game, triggerGame } = useSingleplayerGame();
  const { setPending } = usePending();
  const { getPlayerShips, updatePlayerHits, getPlayerHits, updateEnemyBoard, updatePlayerBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const updateBoards = (ev: MoveMadeEvent) => {
    const { step_result, bot_step, step } = ev;

    if (!isNull(bot_step)) {
      updatePlayerBoard(gameType, bot_step);
      updatePlayerHits(gameType, bot_step);
    }
    if (!isNull(step_result) && !isNull(step)) {
      updateEnemyBoard(gameType, stepResultToBoardEntityMap[step_result], step);
    }
  };

  const generateProofHit = async (step: string) => {
    const ships = getPlayerShips(gameType);
    const hits = getPlayerHits(gameType);

    if (!ships || !hits) {
      return;
    }

    const proofData = await requestProofHit(
      ships,
      step,
      hits.map((item) => item.toString()),
    );

    return proofData;
  };

  const onData = async (ev: MoveMadeEvent) => {
    if (account?.decodedAddress !== ev.player) {
      return;
    }
    try {
      if (!isNull(ev.bot_step)) {
        const proofData = await generateProofHit(ev.bot_step.toString());
        saveProofData(gameType, proofData);
      }
      updateBoards(ev);

      triggerGame();
      setPending(false);
    } catch (err) {
      console.log(err);
    }
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
