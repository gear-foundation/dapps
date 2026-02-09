import { useProgram as useGearJsProgram, useProgramQuery } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

// import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program as PokerProgram } from './poker';
import { Program as PokerFactoryProgram } from './poker-factory';
import { Program as PtsProgram } from './pts';

const usePokerFactoryProgram = () => {
  // ! TODO: remove this after testing
  // const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();
  const pokerFactoryProgramId = '0x85d209a16fdcca5fe78fe4ac0a6ce1fd72551d0ea4d574f891e85a5666eb84bd';
  const { data: program } = useGearJsProgram({ library: PokerFactoryProgram, id: pokerFactoryProgramId });

  return program;
};

const usePokerProgram = () => {
  const location = useLocation();
  // use from location (not useParams) because it uses in hocs before routing
  const gameId = location.pathname.match(/\/game\/([^/]+)/)?.[1];
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

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram, PokerFactoryProgram };
