import { useEffect, useRef } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { StepResult } from '@/app/utils/sails/lib/lib';
import { useSingleplayerGame } from '@/features/singleplayer/hooks/use-singleplayer-game';
import { useProgram } from '@/app/utils/sails';
import { useAccount } from '@gear-js/react-hooks';

type MoveMadeEvent = {
  bot_step: number;
  step: number;
  step_result: StepResult;
  player: string;
};

const config = {
  Missed: 'Boom',
  Injured: 'BoomShip',
  Killed: 'DeadShip',
};

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const gameType = 'single';
  const event = useRef<Promise<() => void> | null>(null);
  const { account } = useAccount();
  const { game, triggerGame } = useSingleplayerGame();
  const { getPlayerShips, updatePlayerHits, getPlayerHits, updateEnemyBoard, updatePlayerBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const updateBoards = (ev: MoveMadeEvent) => {
    const { step_result, bot_step, step } = ev;

    updatePlayerBoard(gameType, bot_step);
    updateEnemyBoard(gameType, config[step_result], step);
    updatePlayerHits(gameType, bot_step);
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

  const moveMadeCallback = async (ev: MoveMadeEvent) => {
    if (account?.decodedAddress !== ev.player) {
      return;
    }
    console.log(ev);
    try {
      const proofData = await generateProofHit(ev.bot_step.toString());

      updateBoards(ev);
      saveProofData(gameType, proofData);

      triggerGame();
    } catch (err) {
      console.log(err);
    }
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
      event.current = program.single.subscribeToMoveMadeEvent((ev: MoveMadeEvent) => moveMadeCallback(ev));
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
