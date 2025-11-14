import { useProgramEvent } from '@gear-js/react-hooks';
import { useSetAtom } from 'jotai';

import { useProgram } from '@/app/utils';

import { useGame } from '../../hooks';
import { stateChangeLoadingAtom } from '../../store';

export type GameFinishedEvent = { game: GameInstance };

export function useEventGameFinishedSubscription() {
  const program = useProgram();
  const { updateGame } = useGame();
  const setIsLoading = useSetAtom(stateChangeLoadingAtom);

  const onData = ({ game }: GameFinishedEvent) => {
    updateGame(game);
    setIsLoading(false);
  };

  useProgramEvent({
    program,
    serviceName: 'ticTacToe',
    functionName: 'subscribeToGameFinishedEvent',
    onData,
  });
}
