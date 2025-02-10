import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { useDnsProgramIds } from '@dapps-frontend/hooks';

import { Program } from '@/app/utils';

const useProgram = () => {
  const { programId } = useDnsProgramIds();
  console.log('programId: ', programId);
  const { data: program } = useGearJsProgram({ library: Program, id: programId });

  return program;
};

export { useProgram };
