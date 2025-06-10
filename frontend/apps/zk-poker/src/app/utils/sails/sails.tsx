import { useProgram as useGearJsProgram, useProgramQuery } from '@gear-js/react-hooks';
import { useParams } from 'react-router-dom';

import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program as PokerProgram } from './poker';
import { Program as PokerFactoryProgram } from './poker-factory';
import { Program as PtsProgram } from './pts';

const usePokerFactoryProgram = () => {
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();
  const { data: program } = useGearJsProgram({ library: PokerFactoryProgram, id: pokerFactoryProgramId });

  return program;
};

const usePokerProgram = () => {
  const { gameId } = useParams();
  const id = gameId ? (gameId as `0x${string}`) : undefined;
  const { data: program } = useGearJsProgram({ library: PokerProgram, id });

  return program;
};

const usePtsProgram = () => {
  const pokerFactoryProgram = usePokerFactoryProgram();

  const { data: ptsProgramId } = useProgramQuery({
    program: pokerFactoryProgram,
    serviceName: 'pokerFactory',
    functionName: 'ptsActorId',
    args: [],
  });

  const { data: program } = useGearJsProgram({ library: PtsProgram, id: ptsProgramId });

  return program;
};

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram };
