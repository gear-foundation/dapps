import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { useLocation } from 'react-router-dom';

import { useDnsProgramIds } from '@dapps-frontend/hooks';
import { useTypedProgramQuery } from '@/features/game/sails/query-utils';

import { Program as PokerProgram } from './poker';
import { Program as PokerFactoryProgram } from './poker-factory';
import { Program as PtsProgram } from './pts';

const asProgramId = (value: unknown): `0x${string}` | undefined =>
  typeof value === 'string' && value.startsWith('0x') ? (value as `0x${string}`) : undefined;

const usePokerFactoryProgram = () => {
  const { pokerFactoryProgramId } = useDnsProgramIds<'pokerFactoryProgramId'>();
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

  const { data: ptsProgramId } = useTypedProgramQuery({
    program: pokerFactoryProgram,
    serviceName: 'pokerFactory',
    functionName: 'ptsActorId',
    args: [],
  });

  const { data: program } = useGearJsProgram({ library: PtsProgram, id: asProgramId(ptsProgramId) });

  return program;
};

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram, PokerFactoryProgram };
