import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { SailsProgram } from './lib';

const useProgram = () => {
  const { programId } = useDnsProgramIds();

  const { data: program } = useGearJsProgram({
    library: SailsProgram,
    id: programId,
  });

  return program;
};

export { useProgram };
