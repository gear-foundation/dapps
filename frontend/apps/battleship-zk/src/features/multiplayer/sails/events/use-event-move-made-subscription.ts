import { useEffect, useRef } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';

import { useShips } from '@/features/zk/hooks/use-ships';
import { useProgram } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { useAccount } from '@gear-js/react-hooks';

type MoveMadeEvent = {
  step: number;
  game_id: string;
  target_address: string;
};

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const gameType = 'multi';
  const event = useRef<Promise<() => void> | null>(null);
  const { account } = useAccount();
  const { game, triggerGame } = useMultiplayerGame();
  const { getPlayerShips, getPlayerHits, updatePlayerHits, updatePlayerBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const generateProofHit = async (ev: MoveMadeEvent) => {
    const ships = getPlayerShips(gameType);
    const hits = getPlayerHits(gameType);

    if (!ships || !hits) {
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
    const { game_id, target_address, step } = ev;

    if (game_id !== game?.admin || target_address !== account?.decodedAddress) {
      return;
    }

    const proofData = await generateProofHit(ev);

    updatePlayerBoard(gameType, step);
    updatePlayerHits(gameType, step);

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
