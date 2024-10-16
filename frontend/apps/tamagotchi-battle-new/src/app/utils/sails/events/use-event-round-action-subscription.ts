import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { Move, Pair, useProgram } from '@/app/utils';
import { useRef, useState } from 'react';

type RoundData = {
  round: number;
  player_1: [string, Move, number];
  player_2: [string, Move, number];
};

export function useEventRoundActionSubscription(pair: Pair) {
  const program = useProgram();
  const { account } = useAccount();
  const [lastMoves, setLastMoves] = useState<{ moves: [Move, Move]; newHealth: [number, number] } | null>(null);

  const resetLastMoves = () => setLastMoves(null);

  const roundRef = useRef<number | null>(null);

  const onData = ({ round, player_1, player_2 }: RoundData) => {
    const players = [pair.player_1, pair.player_2];

    if (players.includes(player_1[0]) && players.includes(player_2[0]) && account && roundRef.current !== round) {
      roundRef.current = round;
      const myData = account.decodedAddress === player_1[0] ? player_1 : player_2;
      const opponentsData = account.decodedAddress === player_1[0] ? player_2 : player_1;

      setLastMoves({ moves: [myData[1], opponentsData[1]], newHealth: [myData[2], opponentsData[2]] });
    }
  };

  useProgramEvent({
    program,
    serviceName: 'battle',
    functionName: 'subscribeToRoundActionEvent',
    onData,
  });

  return { lastMoves, resetLastMoves };
}
