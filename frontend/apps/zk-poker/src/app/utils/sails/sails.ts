import { useProgram as useGearJsProgram, useProgramQuery } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

// import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program as PokerProgram } from './poker';
import { Program as PokerFactoryProgram } from './poker-factory';
import { Program as PtsProgram } from './pts';

const usePokerFactoryProgram = () => {
  // const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();
  const pokerFactoryProgramId = '0x6b6c0f4aeac5268590614e5e04229c4a59a9ac9725d4c5f956b6f9367ef5b1a2';
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
