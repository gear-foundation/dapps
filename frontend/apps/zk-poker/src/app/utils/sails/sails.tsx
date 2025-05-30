import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { useParams } from 'react-router-dom';

// import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program as PokerProgram } from './poker';
import { Program as PokerFactoryProgram } from './poker-factory';
import { Program as PtsProgram } from './pts';

const usePokerFactoryProgram = () => {
  // const { pokerProgramId } = useDnsProgramIds();
  const pokerFactoryProgramId = '0x1fd98c631fc3583aa21f83d172a67ca7f3e46be96273c59a652e207af8d034f5';
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
  // const { ptsProgramId } = useDnsProgramIds();
  const ptsProgramId = '0xe25ad8394efcf5b7f8178471845275cb1767d9cf8091c423dd57b854761d2b58';
  const { data: program } = useGearJsProgram({ library: PtsProgram, id: ptsProgramId });

  return program;
};

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram };
