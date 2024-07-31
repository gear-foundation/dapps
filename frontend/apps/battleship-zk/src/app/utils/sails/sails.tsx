import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { Program } from '@/app/utils/sails/lib/lib';
import { ADDRESS } from '@/app/consts';

const useProgram = () => {
  // TODO: add when swith to dns
  // const { data: id } = useQuery({ queryKey: ['dnsProgramId'], queryFn: getDnsProgramId });
  const { data: program } = useGearJsProgram({ library: Program, id: ADDRESS.GAME });

  return program;
};

export { useProgram };
