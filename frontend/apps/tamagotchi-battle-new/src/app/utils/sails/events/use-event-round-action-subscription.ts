import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { Move, Pair, useProgram } from '@/app/utils';
import { useState } from 'react';

export function useEventRoundActionSubscription(pair?: Pair) {
  const program = useProgram();
  const { account } = useAccount();
  const [lastMoves, setLastMoves] = useState<{ moves: [Move, Move]; newHealth: [number, number] } | null>(null);

  const resetLastMoves = () => setLastMoves(null);

  const onData = ([player1, player2]: [[string, Move, number], [string, Move, number]]) => {
    const players = [pair?.player_1, pair?.player_2];

    if (players.includes(player1[0]) && players.includes(player2[0]) && account) {
      const myData = account.decodedAddress === player1[0] ? player1 : player2;
      console.log('🚀 ~ onData ~ myData:', myData);
      const opponentsData = account.decodedAddress === player1[0] ? player2 : player1;
      console.log('🚀 ~ onData ~ opponentsData:', opponentsData);

      setLastMoves({ moves: [myData[1], opponentsData[1]], newHealth: [myData[2], opponentsData[2]] });
    }
  };

  useProgramEvent({
    program: pair ? program : undefined,
    serviceName: 'battle',
    functionName: 'subscribeToRoundActionEvent',
    onData,
  });

  return { lastMoves, resetLastMoves };
}
