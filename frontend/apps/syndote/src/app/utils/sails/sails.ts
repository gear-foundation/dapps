import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { Program } from './syndote';
import { useDnsProgramIds } from '@dapps-frontend/hooks';

const useProgram = () => {
  const { programId } = useDnsProgramIds();

  const { data: program } = useGearJsProgram({
    library: Program,
    id: programId,
  });

  return program;
};

export { useProgram };
