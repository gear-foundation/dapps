import { useAccount, useProgramEvent } from '@gear-js/react-hooks';
import { Move, Pair, useProgram } from '@/app/utils';
import { useState } from 'react';

export function useEventRoundActionSubscription(pair?: Pair) {
  const program = useProgram();
  const { account } = useAccount();
  const [lastMoves, setLastMoves] = useState<[Move, Move] | null>(null);

  const resetLastMoves = () => setLastMoves(null);

  const onData = ([player1, player2]: [[string, Move], [string, Move]]) => {
    const players = [pair?.player_1, pair?.player_2];

    if (players.includes(player1[0]) && players.includes(player2[0]) && account) {
      const myMove = account.decodedAddress === player1[0] ? player1[1] : player2[1];
      const opponentsMove = account.decodedAddress === player1[0] ? player2[1] : player1[1];
      setLastMoves([myMove, opponentsMove]);
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
