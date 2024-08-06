import { useProgram } from '@/app/utils/sails';
import { useProgramQuery } from '@gear-js/react-hooks';

export const useConfigurationQuery = () => {
  const program = useProgram();

  return useProgramQuery({
    program,
    serviceName: 'admin',
    functionName: 'configuration',
    args: [],
  });
};
