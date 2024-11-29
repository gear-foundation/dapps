import { useVaratubeProgram } from '../sails';
import { useProgramQuery } from '@gear-js/react-hooks';

export const useCurrenciesQuery = () => {
  const program = useVaratubeProgram();

  const { data, refetch, isFetching, error } = useProgramQuery({
    program,
    serviceName: 'varatube',
    functionName: 'currencies',
    args: [],
  });

  return { currencies: data, isFetching, refetch, error };
};
