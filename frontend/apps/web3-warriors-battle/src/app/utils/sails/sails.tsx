import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { Program } from '@/app/utils';
import { useDnsProgramIds } from '@dapps-frontend/hooks';

const useProgram = () => {
  // const { programId } = useDnsProgramIds();
  const programId = '0x65a336761656a606494e330ec8ca260224909a54eca3884f92fec254701b9f19';

  const { data: program } = useGearJsProgram({ library: Program, id: programId });

  return program;
};

export { useProgram };
