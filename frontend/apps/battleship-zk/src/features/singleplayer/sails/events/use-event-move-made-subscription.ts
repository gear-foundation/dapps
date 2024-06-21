import { useEffect, useRef } from 'react';
import { useProofShipHit } from '@/features/zk/hooks/use-proof-ship-hit';
import { useShips } from '@/features/zk/hooks/use-ships';
import { StepResult } from '@/features/game/assets/lib/lib';
import { useSingleplayerGame } from '@/features/singleplayer/hooks/use-singleplayer-game';
import { program } from '@/app/utils/sails';

type MoveMadeEvent = {
  bot_step: number;
  step: number;
  step_result: StepResult;
};

type MarkedShips = {
  [key: string]: 1 | 0;
};

const config = {
  Missed: 'Boom',
  Injured: 'BoomShip',
  Killed: 'DeadShip',
};

const defineDeadShip = (i: number, board: string[]) => {
  const numCols = 5;
  const markedShips: MarkedShips = {};

  const defineDeadShip = (i: number, board: string[]): string[] => {
    markedShips[i] = 1;

    if (board[i + 1] === 'BoomShip' && !markedShips[i + 1] && (i + 1) % numCols !== 0) {
      defineDeadShip(i + 1, board);
    }

    if (board[i - 1] === 'BoomShip' && !markedShips[i - 1] && (i % numCols !== 0 || i === 0)) {
      defineDeadShip(i - 1, board);
    }

    if (board[i + numCols] === 'BoomShip' && !markedShips[i + numCols]) {
      defineDeadShip(i + numCols, board);
    }

    if (board[i - numCols] === 'BoomShip' && !markedShips[i - numCols]) {
      defineDeadShip(i - numCols, board);
    }

    board[i] = 'DeadShip';

    //borders
    if (board[i + 1] === 'Unknown' && (i + 1) % numCols !== 0) {
      board[i + 1] = 'Boom';
    }
    if (board[i - 1] === 'Unknown' && i % numCols !== 0 && i > 0) {
      board[i - 1] = 'Boom';
    }
    if (board[i + numCols] === 'Unknown') {
      board[i + numCols] = 'Boom';
    }
    if (board[i - numCols] === 'Unknown') {
      board[i - numCols] = 'Boom';
    }

    //corners
    if (board[i + 1 + numCols] === 'Unknown' && (i + 1) % numCols !== 0 && (i + 1 + numCols) % numCols !== 0) {
      board[i + 1 + numCols] = 'Boom';
    }
    if (board[i + 1 - numCols] === 'Unknown' && (i + 1) % numCols !== 0 && (i + 1 - numCols) % numCols !== 0) {
      board[i + 1 - numCols] = 'Boom';
    }
    if (board[i - 1 + numCols] === 'Unknown' && i % numCols !== 0 && (i + numCols) % numCols !== 0 && i > 0) {
      board[i - 1 + numCols] = 'Boom';
    }
    if (board[i - 1 - numCols] === 'Unknown' && i % numCols !== 0 && (i - numCols) % numCols !== 0 && i > 0) {
      board[i - 1 - numCols] = 'Boom';
    }

    return board;
  };

  defineDeadShip(i, board);

  return board;
};

export function useEventMoveMadeSubscription() {
  const event = useRef<Promise<() => void> | null>(null);
  const { game, triggerGame } = useSingleplayerGame();
  const { getPlayerShips, getBoard, setBoard } = useShips();
  const { requestProofHit, saveProofData, clearProofData } = useProofShipHit();

  const updatePlayerBoard = (bot_step: number) => {
    const board = getBoard('single', 'player');

    if (!board) {
      return;
    }

    if (board[bot_step] === 'Empty') {
      board[bot_step] = 'Boom';
    }

    if (board[bot_step] === 'Ship') {
      board[bot_step] = 'BoomShip';
    }

    setBoard('single', 'player', board);
  };

  const updateEnemyBoard = (step_result: StepResult, player_step: number) => {
    const board = getBoard('single', 'enemy');

    if (!board) {
      return;
    }

    if (config[step_result] === 'DeadShip') {
      const updatedBoard = defineDeadShip(player_step, board);

      setBoard('single', 'enemy', updatedBoard);
      return;
    }

    board[player_step] = config[step_result];
    setBoard('single', 'enemy', board);
  };

  const updateBoards = (ev: MoveMadeEvent) => {
    const { step_result, bot_step, step } = ev;

    updatePlayerBoard(bot_step);
    updateEnemyBoard(step_result, step);
  };

  const generateProofHit = async (ev: MoveMadeEvent) => {
    const ships = getPlayerShips('single');

    if (!ships) {
      return;
    }

    const proofData = await requestProofHit(ships, ev.bot_step.toString());

    return proofData;
  };

  const moveMadeCallback = async (ev: MoveMadeEvent) => {
    const proofData = await generateProofHit(ev);

    updateBoards(ev);
    saveProofData('single', proofData);

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
      clearProofData('single');
    }
  }, [game]);
}
