import { useProgramEvent } from '@gear-js/react-hooks';
import { useSetAtom } from 'jotai';
import { GameInstance, useProgram } from '@/app/utils';
import { stateChangeLoadingAtom } from '../../store';
import { useGame } from '../../hooks';

export type MoveMadeEvent = { game: GameInstance };

export function useEventMoveMadeSubscription() {
  const program = useProgram();
  const { updateGame } = useGame();
  const setIsLoading = useSetAtom(stateChangeLoadingAtom);

  const onData = ({ game }: MoveMadeEvent) => {
    updateGame(game);
    setIsLoading(false);
  };

  useProgramEvent({
    program,
    serviceName: 'ticTacToe',
    functionName: 'subscribeToMoveMadeEvent',
    onData,
  });
}
