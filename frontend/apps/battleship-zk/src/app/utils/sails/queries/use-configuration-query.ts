import { useProgramQuery } from '@gear-js/react-hooks';

import { useProgram } from '@/app/utils/sails';

export const useConfigurationQuery = () => {
  const program = useProgram();

  return useProgramQuery({
    program,
    serviceName: 'admin',
    functionName: 'configuration',
    args: [],
  });
};
