import { useEffect, useRef } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';

import { useShips } from '@/features/zk/hooks/use-ships';
import { program } from '@/app/utils/sails';
import { useMultiplayerGame } from '../../hooks';
import { useAccount } from '@gear-js/react-hooks';

type MoveMadeEvent = {
  step: number;
  game_id: string;
  target_address: string;
};

export function useEventMoveMadeSubscription() {
  const event = useRef<Promise<() => void> | null>(null);
  const { account } = useAccount();
  const { game, triggerGame } = useMultiplayerGame();
  const { getPlayerShips, getBoard, setBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const updatePlayerBoard = (bot_step: number) => {
    const board = getBoard('multi', 'player');

    if (!board) {
      return;
    }

    if (board[bot_step] === 'Empty') {
      board[bot_step] = 'Boom';
    }

    if (board[bot_step] === 'Ship') {
      board[bot_step] = 'BoomShip';
    }

    setBoard('multi', 'player', board);
  };

  const generateProofHit = async (ev: MoveMadeEvent) => {
    const ships = getPlayerShips('multi');

    if (!ships) {
      return;
    }

    const proofData = await requestProofHit(ships, ev.step.toString());

    return proofData;
  };

  const moveMadeCallback = async (ev: MoveMadeEvent) => {
    const { game_id, target_address, step } = ev;

    if (game_id !== game?.admin || target_address !== account?.decodedAddress) {
      return;
    }

    const proofData = await generateProofHit(ev);

    updatePlayerBoard(step);
    saveProofData('multi', proofData);

    triggerGame();
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
      clearProofData('multi');
    }
  }, [game]);
}
