import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';
import { WarriorProgram } from './warrior';

const useWarriorProgram = (programId: string) => {
  const enabled = programId.startsWith('0x') && programId.length === 66 ? true : false;
  const { data: program } = useGearJsProgram({ library: WarriorProgram, id: programId as `0x${string}`, query: { enabled } });

  return program;
};

export { useWarriorProgram };
