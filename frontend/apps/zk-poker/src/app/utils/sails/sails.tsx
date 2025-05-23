import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

// import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program as PokerProgram } from './poker';
import { Program as PtsProgram } from './pts';

const usePokerProgram = () => {
  // const { pokerProgramId } = useDnsProgramIds();
  const pokerProgramId = '0x5a682a335d49619d65f03ca12006e949945c17bdcc65ba8355e8701cb9a74d97';
  const { data: program } = useGearJsProgram({ library: PokerProgram, id: pokerProgramId });

  return program;
};

const usePtsProgram = () => {
  // const { ptsProgramId } = useDnsProgramIds();
  const ptsProgramId = '0x9fd9ad914d7ec9e6a591e39b785d5f60d4d6510cc7b93668fee9957e648441f1';
  const { data: program } = useGearJsProgram({ library: PtsProgram, id: ptsProgramId });

  return program;
};

export { usePokerProgram, usePtsProgram };
